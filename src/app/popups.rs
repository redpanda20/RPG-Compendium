use crate::mystward::character;

pub enum Popup {
	None,
	LogIn(UserDetails),
	CreateAccount(UserDetails),
	ViewAccount,
	CreateCharacter(CreateCharacterDetails),
	AdvanceCharacter(CharacterAdvanceDetails)
}
impl Default for Popup {
    fn default() -> Self {
        Popup::None
    }
}
impl Popup {
	pub fn is_none(&self) -> bool {
		match self {
			Popup::None => true,
			_ => false,
		}
	}
}

pub struct UserDetails{
	pub username: String,
	pub password: String,
}
impl UserDetails {
	pub fn new(username: String, password:String) -> Self {
		Self {
			username,
			password,
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

pub fn show_signup(parent: &mut super::App, ctx: &egui::Context) -> (bool, Option<egui::InnerResponse<Option<()>>>) {
	let Popup::CreateAccount(details) = &mut parent.current_popup else {
		return (false, None)
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

		created_user = true;
	}

	if !*is_window_open || created_user {
		parent.current_popup = Popup::None
	}

	return (created_user, response)
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

		if let Some((id, size)) = parent.current_user.get_profile_picture(ctx) {
			if ui.add(
				egui::ImageButton::new(id, size)
			).clicked() {
				parent.loader.file_dialog(super::loader::FileUsage::ProfilePicture);
			};
		}		

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

pub struct CreateCharacterDetails{
	pub name: String,
	pub archetype: character::RacialArchetype,
}
impl Default for CreateCharacterDetails {
	fn default() -> Self {
		Self {
			name: String::from(""),
			archetype: character::RacialArchetype::Undecided
		}
	}
}
pub fn show_create_character(parent: &mut super::App, ctx: &egui::Context) -> (bool, Option<egui::InnerResponse<Option<()>>>) {
	let Popup::CreateCharacter(details) = &mut parent.current_popup else {
		return (false, None)
	};

	let is_window_open: &mut bool = &mut true;
	let mut button = false;

	let response = egui::Window::new("Create New Character")
	.anchor(
		egui::Align2::CENTER_CENTER,
		egui::vec2(0.0, 0.0))
	.open(is_window_open)
	.auto_sized()
	.collapsible(false)
	.show(ctx, |ui| {
		ui.label("Character name");
		ui.text_edit_singleline(&mut details.name);

		egui::ComboBox::from_id_source("Race combobox")
			.width(200.0)
			.selected_text(match details.archetype {
				character::RacialArchetype::Undecided => "",
				character::RacialArchetype::Byvine(_) => "Byvine",
				character::RacialArchetype::Clank(_) => "Clank",
				character::RacialArchetype::Human(_) => "Human",
				character::RacialArchetype::MoonElf(_) => "Moon Elf",
				character::RacialArchetype::MystFae(_) => "Myst Fae",
				character::RacialArchetype::Treekin(_) => "Treekin",
				character::RacialArchetype::Wyvren(_) => "Wyvren",
			})
			.show_ui(ui, |ui| {
				ui.selectable_value(&mut details.archetype, character::RacialArchetype::Byvine(character::ByvineClass::Base), "Byvine");
				ui.selectable_value(&mut details.archetype, character::RacialArchetype::Clank(character::ClankClass::Base), "Clank");
				ui.selectable_value(&mut details.archetype, character::RacialArchetype::Human(character::HumanClass::Base), "Human");
				ui.selectable_value(&mut details.archetype, character::RacialArchetype::MoonElf(character::MoonElfClass::Base), "Moon Elf");
				ui.selectable_value(&mut details.archetype, character::RacialArchetype::MystFae(character::MystFaeClass::Base), "MystFae");
				ui.selectable_value(&mut details.archetype, character::RacialArchetype::Treekin(character::TreekinClass::Base), "Treekin");
				ui.selectable_value(&mut details.archetype, character::RacialArchetype::Wyvren(character::WyvrenClass::Base), "Wyvren");

			});		

		let archetype_info = details.archetype.get_variants();

		if !archetype_info.is_empty() {
			let title = details.archetype.to_string();
			egui::ComboBox::from_id_source("Archetype combobox")
				.width(200.0)
				.selected_text(title)
				.show_ui(ui, |ui| {
					for archetype in archetype_info {
						ui.selectable_value(&mut details.archetype, archetype.clone(), archetype.to_string());
					}
				});
		}

		let mut attributes: Vec<(character::Attribute, u8)> = character::attributes::default().to_vec();
		for (attribute, quanity) in character::attributes::from_archetype(&details.archetype) {
			let pos = attributes.iter().position(|(att, _)| att == &attribute);
			match pos {
				Some(index) => attributes[index].1 = attributes[index].1 + quanity,
				None => attributes.push((attribute, quanity)),
			}
		}

		let traits = character::traits::from_archetype(&details.archetype);
		ui.shrink_width_to_current();
		
		ui.columns(2, |column| {
			column[0].vertical_centered(|ui| {
				ui.label(egui::RichText::new("Attributes").size(24.0));
			});
			column[0].vertical(|ui| {
				ui.separator();
		
				if let Some((_, quantity)) = attributes.iter().find(|(att, _)| att == &character::Attribute::Unused) {
					ui.label( egui::RichText::new(format!("{}: {}", character::Attribute::Unused.to_string(), quantity)).size(16.0) );
					ui.add_space(10.0);
				}
		
				for (attribute, quantity) in attributes {
					if attribute == character::Attribute::Unused {continue;}
					ui.label( egui::RichText::new(format!("{}: {}", attribute.to_string(), quantity)).size(16.0) );
				}
			});
			column[1].vertical_centered(|ui| {
				ui.label(egui::RichText::new("Traits").size(24.0));
			});
			column[1].vertical(|ui| {
				ui.separator();
	
				for char_trait in &traits {
					ui.label( egui::RichText::new(char_trait.title.clone()).size(20.0) );
					ui.label( egui::RichText::new(char_trait.text.clone()).size(12.0) );
					ui.add_space(4.0);
				}
			});
		});

		ui.vertical_centered(|ui| {
			button = ui.button("Create Character").clicked();
		});


	});

	let mut created_character = false;
	if button {
		if match &details.archetype	{
			character::RacialArchetype::Undecided => false,
			character::RacialArchetype::Byvine(archetype) => match archetype {
				character::ByvineClass::Base => false,
				_ => true
			},
			character::RacialArchetype::Clank(archetype) => match archetype {
				character::ClankClass::Base => false,
				_ => true
			},
			character::RacialArchetype::Human(archetype) => match archetype {
				character::HumanClass::Base => false,
				_ => true
			},
			character::RacialArchetype::MoonElf(archetype) => match archetype {
				character::MoonElfClass::Base => false,
				_ => true
			},
			character::RacialArchetype::MystFae(archetype) => match archetype {
				character::MystFaeClass::Base => false,
				_ => true
			},
			character::RacialArchetype::Treekin(archetype) => match archetype {
				character::TreekinClass::Base => false,
				_ => true
			},
			character::RacialArchetype::Wyvren(archetype) => match archetype {
				character::WyvrenClass::Base => false,
				_ => true
			}
		} && !details.name.is_empty() {
			parent.current_user.add_character(details.name.clone(), details.archetype.clone());
			parent.current_page = super::pages::Page::CharacterSheet(Default::default());
			created_character = true;
		}
	}

	if !*is_window_open || created_character {
		parent.current_popup = Popup::None
	}

	return (created_character, response)
}

pub struct CharacterAdvanceDetails {
	pub selected_advance: Option<character::Advance>
}
impl Default for CharacterAdvanceDetails {
	fn default() -> Self {
		Self { selected_advance: None }
	}
}
pub fn show_advance_character(parent: &mut super::App, ctx: &egui::Context) -> bool {
	let Popup::AdvanceCharacter(details) = &mut parent.current_popup else {
		return false;
	};

	let is_window_open: &mut bool = &mut true;
	let mut should_continue = false;

	egui::Window::new("Level Up Character")
	.anchor(
		egui::Align2::CENTER_CENTER,
		egui::vec2(0.0, 0.0))
	.open(is_window_open)
	.auto_sized()
	.collapsible(false)
	.show(ctx, |ui| {
		let width = ui.available_width().clamp(300.0, 1000.0);
		ui.set_max_width(width);

		let mut col_index: usize = 0;
		let content_size = egui::vec2(300.0, 200.0);

		let grid_width = ((width - 100.0) / content_size.x) as usize;

		ui.horizontal(|ui| {
			ui.add_space((ui.available_width() - grid_width as f32 * content_size.x - (grid_width - 1) as f32 * 10.0 ) / 2.0 );

			egui::Grid::new("Advance options")
				.max_col_width(content_size.x)
				.spacing(egui::vec2(10.0, 10.0))
				.show(ui, |ui| {

				for advance in character::Advance::all_advances() {
					
					let res = if let Some(selected) = &mut details.selected_advance {
						if match (advance, *selected) {
								(character::Advance::Other, character::Advance::Other) => true,
								(character::Advance::TrainWildMagic, character::Advance::TrainWildMagic) => true,
								(character::Advance::TrainMagicFaeSylviel, character::Advance::TrainMagicFaeSylviel) => true,
								(character::Advance::TrainMartial(_), character::Advance::TrainMartial(_)) => true,
								(character::Advance::Study(_), character::Advance::Study(_)) => true,
								(character::Advance::Tame(_), character::Advance::Tame(_)) => true,
								_ => false,
						} {
							selected.show(ui, content_size)
						} else {
							advance.show_preview(ui, content_size)
						}
					} else {
						advance.show_preview(ui, content_size)
					};

					if res.clicked() {
						details.selected_advance = Some(advance);
					}
					col_index += 1;
					if col_index >= grid_width {
						col_index = 0;
						ui.end_row();
					}
				}
			});

		});

		ui.vertical_centered(|ui| {
			let mut enabled = false;
			if let Some(advance) = &details.selected_advance {
				enabled = match advance {
					character::Advance::Other => true,
					character::Advance::TrainMagicFaeSylviel => true,
					character::Advance::TrainWildMagic => true,
					character::Advance::TrainMartial(option) => option.is_some(),
					character::Advance::Study(option) => option.is_some(),
					character::Advance::Tame(option) => option.is_some(),
				};
				let mut text = String::from("Currently selected: ");
				text += advance.to_string().as_str();
				ui.label(text);
			}
			should_continue = ui.add_enabled(
				enabled,
				egui::Button::new("Confirm"))
				.clicked();
		})
	});

	if should_continue {
		if let Some(character) = parent.current_user.get_active_character() {
			if let Some(advance) = details.selected_advance {
				match advance {
					character::Advance::Other => (),
					character::Advance::TrainWildMagic => {
						character.traits.push(character::traits::wild_magic())
					},
					character::Advance::TrainMagicFaeSylviel => {
						character.traits.push(character::traits::fae_magic_sylviel())
					},
					character::Advance::TrainMartial(martial_skill) => {
						if let Some(martial_skill) = martial_skill {
							let char_trait = match martial_skill {
								character::advances::MartialTrait::DanceOfArrows => character::traits::martial_dance_of_arrows(),
								character::advances::MartialTrait::DanceOfBlades => character::traits::martial_dance_of_blades(),
								character::advances::MartialTrait::DanceOfBlood => {
									for (att, quantity) in &mut character.attributes {
										if att == &character::Attribute::FirstAid {
											*quantity += 1;
											break;
										}
									}					
									character::traits::martial_dance_of_blood()
								},
							};
							character.traits.push(char_trait)
						} else {
							return false
						}
					},			
					character::Advance::Study(lore) => {
						if let Some(lore) = lore {
							let attribute = character::Attribute::Lore(lore);
							character.attributes.push((attribute, 1));
						} else {
							// Interupt as state is invalid
							return false
						}},
					character::Advance::Tame(beast) => {
						if let Some(beast) = beast {
							let char_trait = match beast {
								character::advances::Beast::StarPhoenix => character::traits::tamed_phoenix(),
								character::advances::Beast::ShadowPuma => character::traits::tamed_puma(),
								character::advances::Beast::Kirin => character::traits::tamed_kirin(),
							};
							character.traits.push(char_trait);
						} else {
							return false
						}
					}
				}

				// Add an unused attribute
				for (att, quantity) in &mut character.attributes {
					if att == &character::Attribute::Unused {
						*quantity += 1;
						break;
					}
				}
			}
		}
	}

	if !*is_window_open || should_continue {
		parent.current_popup = Popup::None
	}

	return false
}