async fn async_load_file() -> Result<Vec<u8>, &'static str>{

	let dialog = rfd::AsyncFileDialog::new();

	let optional_file = dialog.pick_file().await;

	if let Some(file) = optional_file {
		let raw_file = file.read().await;
		return Ok(raw_file)
	}
	return Err("Could not retrieve file")
}

pub fn show_menu_bar(parent: &mut super::App, ctx: &egui::Context, frame: &mut eframe::Frame) -> egui::InnerResponse<()> {

	let response =
	egui::TopBottomPanel::top("Menu Bar").show(ctx, |ui| {

		egui::menu::bar(ui, |ui| {
			ui.menu_button("File", |ui| {

				// Close button: Innactive on web
				#[cfg(not(target_arch = "wasm32"))]
				if ui.add(
					egui::Button::new("Close")
					.shortcut_text( egui::Context::format_shortcut(ctx, &parent.shortcuts.shutdown))
					
				).clicked() {
					frame.close()
				}

				// Save button
				if ui.add(
					egui::Button::new("Save")
					.shortcut_text( egui::Context::format_shortcut(ctx, &parent.shortcuts.save))

				).clicked() {
					if let Some(storage) = frame.storage_mut() {
						eframe::App::save(parent, storage);
						ui.close_menu();
					}
				}

				// Load button
				if ui.add(
					egui::Button::new("Load")
					//.shortcut_text( Context::format_shortcut(ctx, &self.shortcuts.save))

				).clicked() {

					// Generate promise to load file
					#[cfg(not(target_arch = "wasm32"))]
					let promise = Some(poll_promise::Promise::from_ready(
						async_std::task::block_on(async_load_file())
					));

					#[cfg(target_arch = "wasm32")]
					let promise = Some(poll_promise::Promise::spawn_async(async_load_file()));

					// Hand promise to parent
					parent.load_promise = promise;

					ui.close_menu();
				}					
			});
		
			// Right hand side
			ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {

				ui.menu_button("User Profile", |ui| {
					ui.label("User name");

					if let Some(texture) = parent.images.clone_profile_picture(ctx) {
						ui.image(texture.id(), texture.size_vec2());
					}

				});

				egui::global_dark_light_mode_switch(ui);
			});
		});


	});
	response
}