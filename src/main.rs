#[macro_use]
extern crate tracing;

mod client;
mod common;
mod server;

use bevy::app::App;
use clap::{Parser, Subcommand};

/// Simple client/server program built on Bevy and Quinn to mess around with networking code.
#[derive(Parser, Debug)]
struct CLI {
    #[command(subcommand)]
    command: Command,
}

/// Actual commands.
#[derive(Subcommand, Debug)]
enum Command {
    /// Start a client process.
    Client(client::ClientArgs),

    /// Start a server process.
    Server(server::ServerArgs),
}

fn main() {
    dotenv::dotenv().ok();

    let args = CLI::parse();

    let mut app = App::new();

    match args.command {
        Command::Client(args) => {
            client::add_plugins(&mut app, args);
        }
        Command::Server(args) => {
            server::add_plugins(&mut app, args);
        }
    }

    app.run()
}
