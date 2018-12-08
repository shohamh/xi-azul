use crossbeam::channel::{Receiver, Sender, unbounded};
use futures::{future, Future, Stream};
use futures::sink::Sink;
use xrl::{AvailablePlugins, Client, ConfigChanged, Frontend, FrontendBuilder, LineCache, PluginStarted, PluginStoped,
          ScrollTo, ServerResult, spawn, Style, ThemeChanged, Update,
          UpdateCmds};

use crate::editor::Editor;
use crate::editor::UIMessage;

pub struct XiClientServiceBuilder(Sender<UIMessage>);

impl XiClientServiceBuilder {
    pub fn new() -> (Self, Receiver<UIMessage>) {
        let (tx, rx) = unbounded();
        (XiClientServiceBuilder(tx), rx)
    }
}

impl FrontendBuilder<XiClientService> for XiClientServiceBuilder {
    fn build(self, client: Client) -> XiClientService {
        // self.0 == XiClientServiceBuilder.0 == UnboundedSender<UIMessage>
        XiClientService { message_queue_sink: self.0, client: client }
    }
}


pub struct XiClientService {
    message_queue_sink: Sender<UIMessage>,
    client: Client,
}

impl XiClientService {
    fn pass_message_to_ui(&mut self, ui_message: UIMessage) -> ServerResult<()> {
        match self.message_queue_sink.send(ui_message) {
            Ok(_) => Box::new(future::ok(())),
            Err(e) => {
                let e = format!("Failed to send core event to UI: {}", e);
                Box::new(future::err(e.into()))
            }
        }
    }
}

impl Frontend for XiClientService {
    fn update(&mut self, update: Update) -> ServerResult<()> {
        println!("received `update` from Xi core:\n{:?}", update);
        self.pass_message_to_ui(UIMessage::Update(update))
        // note that we could send requests/notifications to the core here with `self.client`
//        self.line_cache.update(update);
        /*self.ui_model.text_input.text = self.line_cache.lines()
            .iter()
            .map(|line| line.clone().text)
            .collect::<Vec<String>>()
            .join("");*/
    }
    fn scroll_to(&mut self, scroll_to: ScrollTo) -> ServerResult<()> {
        println!("received `scroll_to` from Xi core:\n{:?}", scroll_to);
        Box::new(future::ok(()))
    }
    fn def_style(&mut self, style: Style) -> ServerResult<()> {
        println!("received `def_style` from Xi core:\n{:?}", style);
        Box::new(future::ok(()))
    }
    fn available_plugins(&mut self, scroll_to: AvailablePlugins) -> ServerResult<()> {
        println!(
            "received `available_plugins` from Xi core:\n{:?}",
            scroll_to
        );
        Box::new(future::ok(()))
    }
    fn update_cmds(&mut self, style: UpdateCmds) -> ServerResult<()> {
        println!("received `update_cmds` from Xi core:\n{:?}", style);
        Box::new(future::ok(()))
    }
    fn plugin_started(&mut self, style: PluginStarted) -> ServerResult<()> {
        println!("received `plugin_started` from Xi core:\n{:?}", style);
        Box::new(future::ok(()))
    }
    fn plugin_stoped(&mut self, style: PluginStoped) -> ServerResult<()> {
        println!("received `plugin_stoped` from Xi core:\n{:?}", style);
        Box::new(future::ok(()))
    }
    fn config_changed(&mut self, style: ConfigChanged) -> ServerResult<()> {
        println!("received `config_changed` from Xi core:\n{:?}", style);
        Box::new(future::ok(()))
    }
    fn theme_changed(&mut self, style: ThemeChanged) -> ServerResult<()> {
        println!("received `theme_changed` from Xi core:\n{:?}", style);
        Box::new(future::ok(()))
    }
}

