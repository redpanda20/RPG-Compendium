// use poll_promise::Promise;

pub struct Loader {
	// pub promises: Vec<Option<Promise<(FileUsage, Vec<u8>)>>>,
	// pub load_promise: Option<Promise<Result<(FileUsage, Vec<u8>), &'static str>>>,
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

impl Loader{
	pub fn file_dialog(&mut self, usage: FileUsage) {
		let (tx, rx) = async_std::channel::unbounded();
		let new_usage = usage.clone();

		self.receiver.push(rx);

		#[cfg(not(target_arch = "wasm32"))]
		async_std::task::spawn(async_file_dialog(tx, new_usage));

		#[cfg(target_arch = "wasm32")]
		wasm_bindgen_futures::spawn_local(async_file_dialog(tx, new_usage));
	}
} 