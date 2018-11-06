extern crate azul;

use azul::prelude::*;
use azul::widgets::text_input::*;

struct AppState {
    text_input: TextInputState,
}

impl Default for AppState {
    fn default() -> Self {
        AppState { text_input: TextInputState::new("Hello, world!") }
    }
}

impl Layout for AppState {
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        TextInput::new()
            .bind(info.window, &self.text_input, &self)
            .dom(&self.text_input)
    }
}

fn main() {
    println!("Hello, world!");

    let app = App::new(AppState::default(), AppConfig::default());
    app.run(
        Window::new(WindowCreateOptions::default(), Css::native()).unwrap(),
    ).unwrap();
}
