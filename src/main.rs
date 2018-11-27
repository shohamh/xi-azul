mod editor;

use azul::prelude::*;
use std::string::String;
use azul::text_layout::*;
use azul::widgets::text_input::*;
use futures::{future, Future, Stream};
use xrl::{LineCache, Client, Frontend, FrontendBuilder, ServerResult, Update, ScrollTo, Style,
          AvailablePlugins, UpdateCmds, PluginStarted, PluginStoped, ConfigChanged, ThemeChanged,
          spawn};

#[derive(Debug)]
struct EditorUIModel {
    text_input: TextInputState,
    selections: Vec<editor::selection::Selection>,
}

#[derive(Debug)]
struct XiFrontend {
    client: Client,
    line_cache: LineCache,
    ui_model: EditorUIModel,
}

struct XiFrontendBuilder;

impl Default for EditorUIModel {
    fn default() -> Self {
        EditorUIModel {
            text_input: TextInputState::new("Hello, world!"),
            selections: Vec::new(),
        }
    }
}

impl Frontend for XiFrontend {
    fn update(&mut self, update: Update) -> ServerResult<()> {
        println!("received `update` from Xi core:\n{:?}", update);
        // note that we could send requests/notifications to the core here with `self.client`
        self.line_cache.update(update);
        self.ui_model.text_input.text = self.line_cache.lines()
                                                       .iter()
                                                       .map(|line| line.clone().text)
                                                       .collect::<Vec<String>>()
                                                       .join("");
        Box::new(future::ok(()))
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

impl FrontendBuilder<XiFrontend> for XiFrontendBuilder {
    fn build(self, client: Client) -> XiFrontend {
        XiFrontend {
            client: client,
            ui_model: EditorUIModel::default(),
            line_cache: LineCache::new()
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

fn text_mouse_down(
    state: &mut AppState<EditorUIModel>,
    event: WindowEvent<EditorUIModel>,
) -> UpdateScreen {
    println!("mouse down event: {:#?}", event.cursor_relative_to_item);
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

fn main() {

    // spawn Xi core
    let (mut client, core_stderr) = spawn("xi-core", XiFrontendBuilder {});

    // All clients must send client_started notification first
    tokio::run(client.client_started(None, None).map_err(|_| ()));

    // start logging Xi core's stderr
    let log_core_errors = core_stderr
        .for_each(|msg| {
            println!("xi-core stderr: {}", msg);
            Ok(())
        })
        .map_err(|_| ());

    ::std::thread::spawn(move || { tokio::run(log_core_errors); });

    // Send a request to open a new view, and print the result
    let open_new_view = client
        .new_view(Some("Cargo.toml".into()))
        .map(|view_name| println!("opened new view: {}", view_name))
        .map_err(|_| ());

    tokio::run(open_new_view);

    // sleep until xi-requests are received
    //    ::std::thread::sleep(::std::time::Duration::new(5, 0));

    let app = App::new(EditorUIModel::default(), AppConfig::default());
    app.run(
        Window::new(WindowCreateOptions::default(), Css::native()).unwrap(),
    ).unwrap();
}
