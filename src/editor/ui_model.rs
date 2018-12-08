use std::string::String;

use azul::prelude::*;
use azul::text_layout::*;
use azul::widgets::text_input::*;
use crossbeam::channel::Receiver;
use futures::future::Future;
use futures::future::ok;
use futures::stream::Stream;
use tokio::runtime::current_thread::TaskExecutor;

use crate::editor::selection::Selection;

use super::UIMessage;

#[derive(Debug)]
pub struct EditorUIModel {
    pub message_queue: Receiver<UIMessage>,
    pub text_input: TextInputState,
    pub selections: Vec<Selection>,
}


impl EditorUIModel {
    pub fn new(message_queue: Receiver<UIMessage>) -> Self {
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
    loop {
        match state.message_queue.try_recv() {
            Ok(ui_message) =>
                match ui_message {
                    UIMessage::Update(update_data) => {
                        state.text_input.text = format!("{:?}", update_data.operations);
                    }
                },
            Err(err) => break
        }
    }

    (UpdateScreen::Redraw, TerminateDaemon::Continue)
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
