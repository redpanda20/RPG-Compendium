use crate::user;

pub struct Loader {
	pub receiver: Vec<async_std::channel::Receiver<(FileUsage, Vec<u8>)>>,
}

impl Default for Loader {
    fn default() -> Self {
        Self { receiver: Vec::new() }
    }
}

#[derive(Clone)]
pub enum FileUsage {
	ProfilePicture,
	CharacterPicture,
	Error
}

async fn async_file_dialog(sender: async_std::channel::Sender<(FileUsage, Vec<u8>)>, usage: FileUsage) {
	let dialog = rfd::AsyncFileDialog::new();
	let optional_file = dialog.pick_file().await;

	let Some(file) = optional_file else {
		return ();
	};
	let raw_file = file.read().await;
	let Ok(_) = sender.send((usage, raw_file)).await else {
		return ();
	};
}

impl Loader {
	pub fn file_dialog(&mut self, usage: FileUsage) {
		let (tx, rx) = async_std::channel::unbounded();
		let new_usage = usage.clone();

		self.receiver.push(rx);

		#[cfg(not(target_arch = "wasm32"))]
		async_std::task::spawn(async_file_dialog(tx, new_usage));

		#[cfg(target_arch = "wasm32")]
		wasm_bindgen_futures::spawn_local(async_file_dialog(tx, new_usage));
	}
	pub fn update(&mut self, ctx: &egui::Context, user: &mut user::User) -> bool {
		let mut request_save = false;

		for reciever in &self.receiver {
			let Ok((file_usage, file_raw)) = reciever.try_recv() else {
				break;
			};
			match file_usage {
				FileUsage::ProfilePicture => {
					if user.is_mutable() && user.is_logged_in() {
						user.update_profile_picture(ctx, file_raw.to_vec());
						request_save = true;
					}
					reciever.close();
				},
				FileUsage::CharacterPicture => {
					if let Some(character) = user.get_active_character() {
						character.update_picture(ctx, file_raw);
						request_save = true;
					}
					reciever.close();
				},
				FileUsage::Error => (),
			}
		}
		self.receiver.retain(|recv| match recv.try_recv() {
			Ok(_) => true,
			Err(e) => match e {
				async_std::channel::TryRecvError::Empty => true,
				async_std::channel::TryRecvError::Closed => false,
		}});

		return request_save
	}
}