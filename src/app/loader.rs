use poll_promise::Promise;
use rfd::FileHandle;

pub struct Loader {
	pub promises: Vec<Option<Promise<(FileUsage, Vec<u8>)>>>,
	// pub load_promise: Option<Promise<Result<(FileUsage, Vec<u8>), &'static str>>>,
}

impl Default for Loader {
    fn default() -> Self {
        Self { promises: Vec::new() }
    }
}

pub enum FileUsage {
	ProfilePicture,
	Error
}

pub async fn async_file_dialog(usage: FileUsage) -> (FileUsage, Vec<u8>) {
	let dialog = rfd::AsyncFileDialog::new();
	let optional_file = dialog.pick_file().await;

	if let Some(file) = optional_file {
		let raw_file = file.read().await;
		return (usage, raw_file)	
	}
	return (FileUsage::Error, Vec::new())

}

impl Loader{
	pub fn file_dialog(&mut self, usage: FileUsage) {
		#[cfg(not(target_arch = "wasm32"))]
		let promise: Option<Promise<(FileUsage, std::vec::Vec<u8>)>> = Some(Promise::from_ready(
			async_std::task::block_on(async_file_dialog(usage))
		));
	
		#[cfg(target_arch = "wasm32")]
		let promise = Some(poll_promise::Promise::spawn_async(async_file_dialog(usage)));
	
		// Hand promise to parent
		self.promises.push(promise);
	}

	pub fn load_from(&mut self, usage: FileUsage, file: FileHandle) {

	}
} 