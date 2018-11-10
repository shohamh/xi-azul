extern crate azul;

use editor;
use azul::prelude::*;
use azul::widgets::text_input::*;

#[derive(Debug)]
struct EditorModel {
    text_input: TextInputState,
    selections: Vec<editor::Selection>
}

impl Default for EditorModel {
    fn default() -> Self {
        EditorModel { text_input: TextInputState::new("Hello, world!") }
    }
}

impl Layout for EditorModel {
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        TextInput::new()
            .bind(info.window, &self.text_input, &self)
            .dom(&self.text_input)
            .with_callback(On::MouseDown, Callback(text_mouse_down))
    }
}

fn text_mouse_down(state: &mut AppState<EditorModel>, event: WindowEvent<EditorModel>) -> UpdateScreen {
    println!("event: {:#?}", event);
    UpdateScreen::Redraw
}

fn main() {
    let app = App::new(EditorModel::default(), AppConfig::default());
    app.run(
        Window::new(WindowCreateOptions::default(), Css::native()).unwrap(),
    ).unwrap();
}
