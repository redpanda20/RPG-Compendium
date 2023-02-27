pub mod app;
pub use app::App;

// Local build
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let image: image::DynamicImage = image::io::Reader::open(
        std::path::Path::new("assets/icons-192.png"))
        .unwrap()
		.decode().unwrap();
    let width = image.width();
    let height = image.height();
    let rgba = image.into_bytes();

    let native_options = eframe::NativeOptions {
        icon_data: Some(eframe::IconData{ rgba, width, height }),
        ..Default::default()
    };
    eframe::run_native(
        "My Test Application",
        native_options,
        Box::new(|_cc| Box::new(App::new(_cc)))
    )
}

// Web build (Using trunk)
#[cfg(target_arch = "wasm32")]
fn main() {
	// Make sure panics are logged using `console.error`.
	console_error_panic_hook::set_once();

	tracing_wasm::set_as_global_default();
	
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async move {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|_cc| Box::new(App::new(_cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}