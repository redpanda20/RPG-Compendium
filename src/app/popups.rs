use super::images;

#[derive(PartialEq)]
pub enum Popup {
	None,
	LogIn(UserDetails),
	CreateAccount(UserDetails),
	ViewAccount,
}
impl Default for Popup {
    fn default() -> Self {
        Popup::None
    }
}

#[derive(PartialEq)]
pub struct UserDetails{
	pub username: String,
	pub password: String,
}
impl UserDetails {
	pub fn new(username: String, password:String) -> Self {
		Self {
			username: username,
			password: password,
		}
	}
}
pub fn show_login(parent: &mut super::App, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>>{
	let Popup::LogIn(details) = &mut parent.current_popup else {
		return None
	};
	let is_window_open: &mut bool = &mut true;

	let mut user: bool = false;
	let mut pass: bool = false;
	let mut enter: bool = false;

	let mut new_account: bool = false;

	let response = egui::Window::new("Login")
	.anchor(
		egui::Align2::CENTER_CENTER,
		egui::vec2(0.0, 0.0))
	.open(is_window_open)
	.auto_sized()
	.collapsible(false)
	.show(ctx, |ui| {

		ui.label("Log In Here");

		user = ui.add(
			egui::widgets::TextEdit::singleline(&mut details.username)
				.hint_text("Username")	
		).lost_focus();

		pass = ui.add(
			egui::widgets::TextEdit::singleline(&mut details.password)
				.hint_text("Password")	
				.password(true)
		).lost_focus();

		ui.horizontal(|ui| {
			enter = ui.button("Enter").clicked();	
			if ui.button("Create account").clicked() {
				new_account = true
			}
		});

	});

	let mut logged_in = false;

	if new_account {
		parent.current_popup = Popup::CreateAccount(
			UserDetails::new(
				details.username.clone(),
				details.password.clone()))
	} else if (user || pass) && ctx.input(|i| i.key_pressed(egui::Key::Enter)) || enter {

		for (_key, user) in parent.past_users.clone() {
			if details.username == user.username {
				let mut temp_user = user.clone();

				if temp_user.log_in(details.password.clone()) {
					parent.current_user = temp_user;
					logged_in = true;
					break;	
				}
			}
		}
	}

	if !*is_window_open || logged_in {
		parent.current_popup = Popup::None;
	}

	return response
}

pub fn show_signup(parent: &mut super::App, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>>{
	let Popup::CreateAccount(details) = &mut parent.current_popup else {
		return None
	};

	let is_window_open: &mut bool = &mut true;

	let mut user: bool = false;
	let mut pass: bool = false;
	let mut enter: bool = false;

	let response = egui::Window::new("Login")
	.anchor(
		egui::Align2::CENTER_CENTER,
		egui::vec2(0.0, 0.0))
	.open(is_window_open)
	.auto_sized()
	.collapsible(false)
	.show(ctx, |ui| {

		ui.label("Create Account");

		user = ui.add(
			egui::widgets::TextEdit::singleline(&mut details.username)
				.hint_text("Username")	
		).lost_focus();

		pass = ui.add(
			egui::widgets::TextEdit::singleline(&mut details.password)
				.hint_text("Password")	
				.password(true)
		).lost_focus();

		ui.horizontal(|ui| {
			enter = ui.button("Create account").clicked();
		});
	});

	let mut created_user = false;
	if (user || pass) && ctx.input(|i| i.key_pressed(egui::Key::Enter)) || enter {
		parent.current_user = super::user::User::new(
			details.username.clone(),
			details.password.clone());

		created_user = true
	}

	if !*is_window_open || created_user {
		parent.current_popup = Popup::None
	}

	return response
}

pub fn show_account(parent: &mut super::App, ctx: &egui::Context) {

	let is_window_open: &mut bool = &mut true;

	let mut logout: bool = false;

	egui::Window::new("View Account Details")
	.anchor(
		egui::Align2::CENTER_CENTER,
		egui::vec2(0.0, 0.0))
	.open(is_window_open)
	.auto_sized()
	.collapsible(false)
	.show(ctx, |ui| {

		ui.horizontal(|ui| {
			ui.label(egui::RichText::new(parent.current_user.username.clone()).strong());

			ui.label(egui::RichText::new("Logged in").weak());
		});

		let (id, size) = parent.current_user.get_profile_picture().unwrap_or_else( ||
			images::StaticSvg::new_with_size(
				String::from("no_image"),
				images::NO_IMAGE.to_vec(),
				[128, 128])
				.get(ctx)
		);
		if ui.add(
			egui::ImageButton::new(id, size)
		).clicked() {
			parent.loader.file_dialog(super::loader::FileUsage::ProfilePicture);
		};

		logout = ui.button("Log Out").clicked();
	});

	if logout {
		let mut temp_user = parent.current_user.clone();
		temp_user.log_out();
		let key = temp_user.username.clone();

		parent.past_users.insert(key, temp_user);

		parent.current_user = super::user::User::default();
	}

	if !*is_window_open || logout {
		parent.current_popup = Popup::None
	}
}
