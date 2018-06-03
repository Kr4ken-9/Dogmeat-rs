#[macro_use] extern crate serenity;
#[macro_use] extern crate serde_derive;
extern crate toml;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use std::io;

mod config;
mod commands;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    if !config::check_config("../config.toml") { // If there's no config, create one
        println!("Please enter your token:");

        let mut token = String::new();
        io::stdin().read_line(&mut token)
            .expect("Error reading token");

        token.pop(); // Input has a trailing newline, this removes it
        let config = config::Config {
            token
        };
        config::save_config(String::from("../config.toml"), config);
    }

    let config = config::load_config(String::from("../config.toml"));

    // Login with a bot token from the config
    let mut client = Client::new(&config.token, Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .cmd("ping", ping)
        .cmd("ban", commands::ban::ban));

    // start listening for events by starting a single shard
    match client.start() {
        Err(why) => println!("An error occurred while running the client: {:?}", why),
        Ok(asdf) => println!("Client connected successfully!"),
    }
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});
