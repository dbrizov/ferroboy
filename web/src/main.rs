#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
mod audio;

#[cfg(target_arch = "wasm32")]
fn main() {
    app::run();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("ferroboy_web targets the browser; run `trunk serve` in web/");
}
