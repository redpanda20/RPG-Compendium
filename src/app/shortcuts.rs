pub struct Shortcuts {
	#[allow(dead_code)]
	pub shutdown: egui::KeyboardShortcut,
	pub save: egui::KeyboardShortcut,
}

impl Default for Shortcuts {

	fn default() -> Self {
		Self {
			shutdown: egui::KeyboardShortcut::new(egui::Modifiers::CTRL | egui::Modifiers::SHIFT, egui::Key::Q),
			save: egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::S),
		}
	}
}
