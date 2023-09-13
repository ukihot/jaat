mod routes;
use routes::App;

fn main() {
    // Initialization of logger
    wasm_logger::init(wasm_logger::Config::default());
    // launch the web app
    dioxus_web::launch(App);
}
