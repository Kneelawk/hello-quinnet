#[allow(unused_imports)]
#[macro_use]
extern crate tracing;

#[cfg(feature = "client")]
mod client;
mod common;
mod server;

use bevy::app::App;
use clap::{Parser, Subcommand};

/// Simple client/server program built on Bevy and Quinn to mess around with networking code.
#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Actual commands.
#[derive(Subcommand, Debug)]
enum Command {
    /// Start a client process.
    #[cfg(feature = "client")]
    Client(client::ClientArgs),

    /// Start a server process.
    Server(server::ServerArgs),
}

fn main() {
    dotenv::dotenv().ok();

    let args = Cli::parse();

    let mut app = App::new();

    common::setup(&mut app);

    match args.command {
        #[cfg(feature = "client")]
        Command::Client(args) => {
            client::setup(&mut app, args);
        }
        Command::Server(args) => {
            server::setup(&mut app, args);
        }
    }

    app.run();
}
