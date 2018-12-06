use std::string::String;
use azul::text_layout::*;
use azul::widgets::text_input::*;
use azul::prelude::*;
use crate::editor::selection::Selection;
use futures::sync::mpsc::UnboundedReceiver;
use super::UIMessage;
use futures::stream::Stream;
use futures::future::ok;
use futures::future::Future;
use futures::Async;

#[derive(Debug)]
pub struct EditorUIModel {
    pub message_queue: UnboundedReceiver<UIMessage>,
    pub text_input: TextInputState,
    pub selections: Vec<Selection>,
}


impl EditorUIModel {
    pub fn new(message_queue: UnboundedReceiver<UIMessage>) -> Self {
        EditorUIModel {
            text_input: TextInputState::default(),
            selections: vec![],
            message_queue: message_queue
        }
    }
}


impl Layout for EditorUIModel {
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        TextInput::new()
            .bind(info.window, &self.text_input, &self)
            .dom(&self.text_input)
            .with_callback(On::MouseDown, Callback(text_mouse_down))
            .with_callback(On::MouseOver, Callback(text_mouse_over))
            .with_callback(On::MouseUp, Callback(text_mouse_up))
    }
}

fn xi_client_daemon(state: &mut EditorUIModel, _app_resources: &mut AppResources) -> (UpdateScreen, TerminateDaemon) {
    match state.message_queue.by_ref().collect().poll() {
        Ok(Async::Ready(ui_messages)) => {
            for ui_message in ui_messages {
                match ui_message {
                    UIMessage::Update(update_data) => {
                        state.text_input.text = format!("{:?}", update_data.operations);
                    }
                };
            };
            (UpdateScreen::DontRedraw, TerminateDaemon::Continue)
        },
        Ok(Async::NotReady) => (UpdateScreen::DontRedraw, TerminateDaemon::Continue),
        Err(e) => (UpdateScreen::DontRedraw, TerminateDaemon::Terminate)
    }
}

fn text_mouse_down(
    state: &mut AppState<EditorUIModel>,
    event: WindowEvent<EditorUIModel>,
) -> UpdateScreen {
    println!("mouse down event: {:#?}", event.cursor_relative_to_item);
    state.add_daemon(Daemon::unique(DaemonCallback(xi_client_daemon)));
    UpdateScreen::Redraw
}

fn text_mouse_over(
    state: &mut AppState<EditorUIModel>,
    event: WindowEvent<EditorUIModel>,
) -> UpdateScreen {
    println!("mouse over event: {:#?}", event.cursor_relative_to_item);
    UpdateScreen::Redraw
}

fn text_mouse_up(
    state: &mut AppState<EditorUIModel>,
    event: WindowEvent<EditorUIModel>,
) -> UpdateScreen {
    println!("mouse up event: {:#?}", event.cursor_relative_to_item);
    // layout_text();
    UpdateScreen::Redraw
}
