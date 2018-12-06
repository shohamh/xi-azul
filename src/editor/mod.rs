mod selection;
mod wheel;
mod ui_model;
mod ui_message;

use ui_model::EditorUIModel;
use futures::future::Future;
use futures::sync::mpsc::*;
use xrl::LineCache;
use xrl::Client;
pub use ui_message::UIMessage;

#[derive(Debug)]
pub struct Editor {
    pub ui_model: EditorUIModel,
    xi_client: Client,
    line_cache: LineCache,
}

impl Editor {
    pub fn new(client: Client, message_queue: UnboundedReceiver<UIMessage>) -> Editor {
        Editor {
            ui_model: ui_model::EditorUIModel::new(message_queue),
            xi_client: client,
            line_cache: LineCache::new()
        }
    }

    pub fn new_view(&mut self, file_path: String) {
        // Send a request to open a new view, and print the result
        let open_new_view = self.xi_client
            .new_view(Some(file_path))
            .map(|view_name| println!("opened new view: {}", view_name))
            .map_err(|_| ());

        tokio::run(open_new_view);
    }
}
