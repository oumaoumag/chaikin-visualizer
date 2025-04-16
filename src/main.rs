use ::chaikin::run_app;
use macroquad::window::Conf;


fn window_conf() -> Conf {
    Conf {
        window_title: "Chaikin's Algorithm".to_owned(),
        window_width: 1024,
        window_height: 768,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    run_app().await;
}
