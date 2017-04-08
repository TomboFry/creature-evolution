use piston_window::*;
use piston::input::{Button, Motion};
use piston::input::mouse::MouseButton;
use conrod::text::font;
use gui::GUIState;
use modal::Modal;
use population::Population;
use rand::{Rng, StdRng};
use optimisationmethods::OptimisationMethod;
use optimisationmethods::hill_climbing::HillClimbing;
use optimisationmethods::genetic_algorithm::GeneticAlgorithm;
use optimisationmethods::simulated_annealing::SimulatedAnnealing;
// use creature::Creature;

pub struct UIData {
	// Mouse position
	pub mouse_x: f64, pub mouse_y: f64,

	// Is the mouse currently being pressed?
	pub mouse_left_down: bool, pub mouse_right_down: bool,

	// Window dimensions and speed to run at
	pub width: u32, pub height: u32, pub fps: u64,

	// Window title (and main menu title)
	pub title: &'static str,

	// Which page should we draw?
	pub gui_state: GUIState,

	// Rng object to create random numbers
	pub rng: StdRng,

	// Generation Test Options
	pub generation_size: usize,
	pub use_genetic_algorithm: bool,
	pub use_simulated_annealing: bool,
	pub use_hill_climbing: bool,
	pub total_generations: usize,

	pub spectate_method: usize,
	pub spectate_generation: usize,
	pub spectate_creature: usize,

	pub draw_simulation: bool,
	pub simulation_frame: u32,
	pub current_fitness: f32,
	pub optmethods: Vec<Box<OptimisationMethod>>,

	// Modal information
	pub modal_visible: bool,
	pub modal_struct: Option<Modal>
}

impl UIData {
	pub fn new(title: &'static str, win_w: u32, win_h: u32, frames: u64) -> UIData {
		match StdRng::new() {
			// Very unlikely to fail but just in case it does, this will close the program before it even really begins
			Err (err) => {
				panic!("{}", err);
			},
			Ok(val) => UIData {
				mouse_x: 0.0, mouse_y: 0.0,
				mouse_left_down: false, mouse_right_down: false,
				width: win_w, height: win_h, fps: frames,
				title: title,
				gui_state: GUIState::Menu,
				rng: val,
				generation_size: 100,
				use_genetic_algorithm: true,
				use_simulated_annealing: false,
				use_hill_climbing: false,
				total_generations: 0,
				spectate_method: 0,
				spectate_generation: 0,
				spectate_creature: 0,
				draw_simulation: false,
				simulation_frame: 0,
				current_fitness: 0.0,
				optmethods: Vec::with_capacity(3),
				modal_visible: false,
				modal_struct: None
			}
		}
	}

	pub fn modal_new(&mut self, title: String, message: String, btn_a_label: Option<String>, btn_b_label: Option<String>) {
		let mut button_a_label = "Okay".to_string();
		let mut button_b_label = "Close".to_string();

		if let Some(lbl_a) = btn_a_label { button_a_label = lbl_a; }
		if let Some(lbl_b) = btn_b_label { button_b_label = lbl_b; }

		self.modal_struct = Some(Modal {
			title: title,
			message: message,
			button_a_label: button_a_label,
			button_b_label: button_b_label
		});
		self.modal_visible = true;
	}

	pub fn modal_close(&mut self) {
		self.modal_visible = false;
		self.modal_struct = None;
	}

	pub fn event(&mut self, event: &Input) {
		match event {
			// Mouse and Keyboard Down
			&Input::Press(button)  => {
				if button == Button::Mouse(MouseButton::Left) {
					self.mouse_left_down = true;
				} else if button == Button::Mouse(MouseButton::Right) {
					self.mouse_right_down = true;
				}
			},

			// Mouse and Keyboard Up
			&Input::Release(button) => {
				if button == Button::Mouse(MouseButton::Left) {
					self.mouse_left_down = false;
				} else if button == Button::Mouse(MouseButton::Right) {
					self.mouse_right_down = false;
				}
			},

			// Mouse and Scroll Change
			&Input::Move(x) => {
				match x {
					Motion::MouseCursor(mx, my) => {
						self.mouse_x = mx;
						self.mouse_y = my;
					},
					_ => {}
				}
			},

			// Window Resize
			&Input::Resize(x, y) => {
				self.width = x;
				self.height = y;
			},

			_ => {}
		};
	}

	pub fn init_tests(&mut self) {

		if !self.use_genetic_algorithm && !self.use_hill_climbing && !self.use_simulated_annealing {
			return self.modal_new("Error".to_string(), "Please select at least one optimisation method".to_string(), None, None);
		}

		let population = Population::new(self.generation_size, &mut self.rng);

		if self.use_genetic_algorithm {
			self.optmethods.push(GeneticAlgorithm::new(population.clone()));
		}
		if self.use_hill_climbing {
			self.optmethods.push(HillClimbing::new(population.clone()));
		}
		if self.use_simulated_annealing {
			self.optmethods.push(SimulatedAnnealing::new(population));
		}

		self.gui_state = GUIState::Generations;
		self.set_creature_random();
		self.draw_simulation = false;
	}

	pub fn generation_single(&mut self) {
		for method in &mut self.optmethods {
			method.generation_single(&mut self.rng);
		}

		// self.spectate_creature = 0;
		self.spectate_generation += 1;
		self.total_generations += 1;
		self.simulation_frame = 0;
	}

	pub fn set_creature(&mut self, method: usize, index: usize, generation: usize) {
		self.reset_simulation();
		let data = self.optmethods[method].get_data();
		self.spectate_creature = index;
		self.current_fitness = data.generations[generation].creatures[self.spectate_creature].fitness;
	}

	pub fn set_creature_random(&mut self) {
		let index = self.rng.gen_range(0, self.generation_size as usize);
		let gen = self.spectate_generation;
		let mtd = self.spectate_method;
		self.set_creature(mtd, index, gen);
	}

	pub fn reset_simulation(&mut self) {
		for method in &mut self.optmethods {
			let mut data = method.get_data_mut();
			data.generations[self.spectate_generation].creatures[self.spectate_creature].reset_position();
		}
		self.simulation_frame = 0;
	}

	pub fn reset_optmethods(&mut self) {
		self.optmethods.clear();
		self.total_generations = 0;
		self.spectate_generation = 0;
		self.spectate_creature = 0;
		self.draw_simulation = false;
		self.simulation_frame = 0;
	}
}

pub struct Fonts {
	pub regular: font::Id,
	pub bold: font::Id,
	pub italic: font::Id,
	pub bold_italic: font::Id
}
