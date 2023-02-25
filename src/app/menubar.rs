use super::images;

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

				).clicked() {

					parent.loader.file_dialog(super::loader::FileUsage::ProfilePicture);

					ui.close_menu();
				}					
			});
		
			// Right hand side
			ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {

				let has_account = super::images::StaticSvg::new(
					String::from("has_account"),
					parent.images.account.clone(),
					// std::fs::read("assets\\account.svg").unwrap(),

					);

				let no_account = super::images::StaticSvg::new(
					String::from("no_account"),
					parent.images.account_none.clone(),
					// std::fs::read("assets\\account_none.svg").unwrap(),
					);

				let (id, size) = match &mut parent.current_user {
					super::user::CurrentUser::LoggedIn(_) => {
						has_account.get(ctx, ctx.style().visuals.dark_mode)
					}
					super::user::CurrentUser::Empty(_) => {
						no_account.get(ctx, ctx.style().visuals.dark_mode)
					}
				};
				
				ui.menu_image_button(id, size,|ui| {
					match &mut parent.current_user {
		// User is logged in
						super::user::CurrentUser::LoggedIn(user) => {

								ui.label(user.username.clone());
								ui.label(egui::RichText::new("Currently logged in").weak());

								let (id, size) = user.get_profile_picture(ctx).unwrap_or_else( ||
									images::StaticSvg::new(
										String::from("no profile picture"),
										parent.images.no_profile_picture.clone())
									.get(ctx, ctx.style().visuals.dark_mode)
								);

								if ui.add(
									egui::widgets::ImageButton::new(id, size)
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

				let light_dark_toggle = super::images::StaticSvg::new_precalculated(
					String::from("light-dark-toggle"),
						parent.images.toggle_off.clone(),
						parent.images.toggle_on.clone(),
					// std::fs::read("assets\\toggle_off_FILL0.svg").unwrap(),
					// std::fs::read("assets\\toggle_on_FILL1.svg").unwrap(),	
				);
				let (button_id, size) = light_dark_toggle.get(ctx, ctx.style().visuals.dark_mode);

				if ui.add(
					egui::ImageButton::new(
						button_id,
						size
					)
				).clicked() {
					if ctx.style().visuals.dark_mode {
						ctx.set_visuals(egui::Visuals::light())
					} else {
						ctx.set_visuals(egui::Visuals::dark())
					}
				}
			});
		});
	});
	response
}