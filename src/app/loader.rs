use poll_promise::Promise;

pub struct Loader {
	pub load_promise: Option<Promise<Result<(FileUsage, Vec<u8>), &'static str>>>,
}

impl Default for Loader {
    fn default() -> Self {
        Self { load_promise: None }
    }
}

pub enum FileUsage {
	ProfilePicture,
	_GeneralImage,
}

pub async fn async_load_file(usage: FileUsage) -> Result<(FileUsage, Vec<u8>), &'static str>{

	let dialog = rfd::AsyncFileDialog::new();

	let optional_file = dialog.pick_file().await;

	if let Some(file) = optional_file {
		let raw_file = file.read().await;
		return Ok((usage, raw_file))
	}
	return Err("Could not retrieve file")
}

impl Loader{
	pub fn load_as(&mut self, usage: FileUsage) {
		#[cfg(not(target_arch = "wasm32"))]
		let promise = Some(poll_promise::Promise::from_ready(
			async_std::task::block_on(async_load_file(usage))
		));
	
		#[cfg(target_arch = "wasm32")]
		let promise = Some(poll_promise::Promise::spawn_async(async_load_file(usage)));
	
		// Hand promise to parent
		self.load_promise = promise
	}
} 