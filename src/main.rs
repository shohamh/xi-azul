mod editor;
use azul::prelude::*;
use azul::widgets::text_input::*;
use azul::text_layout::*;

#[derive(Debug)]
struct EditorModel {
    text_input: TextInputState,
    selections: Vec<editor::selection::Selection>
}

impl Default for EditorModel {
    fn default() -> Self {
        EditorModel { text_input: TextInputState::new("Hello, world!"), selections: Vec::new() }
    }
}

impl Layout for EditorModel {
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        TextInput::new()
            .bind(info.window, &self.text_input, &self)
            .dom(&self.text_input)
            .with_callback(On::MouseDown, Callback(text_mouse_down))
            .with_callback(On::MouseOver, Callback(text_mouse_over))
            .with_callback(On::MouseUp, Callback(text_mouse_up))
    }
}

fn text_mouse_down(state: &mut AppState<EditorModel>, event: WindowEvent<EditorModel>) -> UpdateScreen {
    println!("mouse down event: {:#?}", event.cursor_relative_to_item);
    UpdateScreen::Redraw
}

fn text_mouse_over(state: &mut AppState<EditorModel>, event: WindowEvent<EditorModel>) -> UpdateScreen {
    println!("mouse over event: {:#?}", event.cursor_relative_to_item);
    UpdateScreen::Redraw
}

fn text_mouse_up(state: &mut AppState<EditorModel>, event: WindowEvent<EditorModel>) -> UpdateScreen {
    println!("mouse up event: {:#?}", event.cursor_relative_to_item);
    layout_text();
    UpdateScreen::Redraw
}

fn main() {
    let app = App::new(EditorModel::default(), AppConfig::default());
    app.run(
        Window::new(WindowCreateOptions::default(), Css::native()).unwrap(),
    ).unwrap();
}
