use super::images;

pub fn show_menu_bar(parent: &mut super::App, ctx: &egui::Context, frame: &mut eframe::Frame) -> egui::InnerResponse<()> {
	
	let home = super::images::StaticSvg::new(
		String::from("Home"), parent.images.home.clone())
		.get(ctx);

	let dark_mode = super::images::StaticSvg::new_precalculated(
		String::from("light-dark-toggle"),
		parent.images.toggle_off.clone(),
		parent.images.toggle_on.clone())
		.get(ctx);

	let vert = super::images::StaticSvg::new(
		String::from("vert"),
		parent.images.more_vert.clone())
		.get(ctx);

	let account = super::images::StaticSvg::new(
		String::from("has_account"),
		parent.images.account.clone())
		.get(ctx);

	let response =
	egui::TopBottomPanel::top("Menu Bar").show(ctx, |ui| {

		egui::menu::bar(ui, |ui| {

			ui.menu_image_button(home.0, home.1, |_ui| {

			});
		
	// Right hand side
			ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {	

				ui.menu_image_button(vert.0, vert.1, |ui| {

					if ui.add(
						egui::ImageButton::new(
							dark_mode.0,
							dark_mode.1,
						)
					).clicked() {
						if ctx.style().visuals.dark_mode {
							ctx.set_visuals(egui::Visuals::light())
						} else {
							ctx.set_visuals(egui::Visuals::dark())
						}
					}

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
					).clicked() {
						parent.loader.file_dialog(super::loader::FileUsage::ProfilePicture);
						ui.close_menu();
					}		

				});

										
				ui.menu_image_button(account.0, account.1,|ui| {
					match &mut parent.current_user {
		// User is logged in
						super::user::CurrentUser::LoggedIn(user) => {

								ui.label(user.username.clone());
								ui.label(egui::RichText::new("Currently logged in").weak());

								let profile = user.get_profile_picture(ctx).unwrap_or_else( ||
									images::StaticSvg::new(
									String::from("no profile picture"),
									parent.images.no_profile_picture.clone())
									.get(ctx));

								if ui.add(
									egui::widgets::ImageButton::new(profile.0, profile.1)
								)
								.on_hover_text("Double click to edit")
								.double_clicked() {
									parent.loader.file_dialog(super::loader::FileUsage::ProfilePicture);
								};

								if ui.add(
									egui::Button::new("Logout")
								).clicked() {
									let new_user = super::user::NewUser::default();
									parent.current_user = super::user::CurrentUser::Empty(new_user);
								};

						},
		// User is not logged in
						super::user::CurrentUser::Empty(new_user) => {
							ui.label("Create account");
							ui.text_edit_singleline(&mut new_user.username);
							ui.text_edit_singleline(&mut new_user.password);

							if ui.add(
								egui::Button::new("Enter")
							).clicked() {
								if !new_user.username.is_empty() && !new_user.password.is_empty() {
									let user = super::user::User::new(
										new_user.username.clone(),
										new_user.password.clone()
									);
									parent.current_user = super::user::CurrentUser::LoggedIn(user)};
							};
						},
					}
				});


			});
		});
	});
	response
}