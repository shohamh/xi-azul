#![feature(uniform_paths)]

mod editor;
mod xi_frontend;

use azul::prelude::*;
use editor::*;
use xi_frontend::*;
use xrl::spawn;
use futures::{future, Future, Stream};

fn main() {

    let (xi_client_service_builder, message_queue_rx) = XiClientServiceBuilder::new();
    // spawn Xi core
    let (mut xi_client, xi_core_stderr) = spawn("xi-core", xi_client_service_builder);

    // Inform the core that our client started
    tokio::run(xi_client.client_started(None, None).map_err(|_| ()));

    // start logging Xi core's stderr
    let log_xi_core_errors = xi_core_stderr
        .for_each(|msg| {
            println!("xi-core stderr: {}", msg);
            Ok(())
        })
        .map_err(|_| ());

    ::std::thread::spawn(move || { tokio::run(log_xi_core_errors); });

    let mut editor = Editor::new(xi_client, message_queue_rx);

    editor.new_view("Cargo.toml".to_string());

    // sleep until xi-requests are received
    //    ::std::thread::sleep(::std::time::Duration::new(5, 0));

    let app = App::new(editor.ui_model, AppConfig::default());
    app.run(
        Window::new(WindowCreateOptions::default(), Css::native()).unwrap(),
    ).unwrap();
}
