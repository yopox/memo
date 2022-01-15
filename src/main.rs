mod roll;
mod stats;
mod players;

use std::env;
use discord::Discord;
use discord::model::{Event, Message};

fn main() {
    let discord = Discord::from_bot_token(&env::var("DISCORD_TOKEN").expect("Expected token"))
        .expect("login failed");

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                if message.author.name == "aria_memo" { continue }
                parse_message(message, &discord)
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Receive error: {:?}", err),
        }
    }
}

fn parse_message(message: Message, discord: &Discord) {
    let content = message.content.clone();
    let command = content.split_whitespace().nth(0);
    if command.is_none() { return; }
    let command = command.unwrap();

    match command {
        "roll" => roll(message, discord),
        "test" => test(message, discord),
        _ => {}
    }
}

fn roll(message: Message, discord: &Discord) {
    match roll::roll(&message.content.strip_prefix("roll").unwrap().to_string()) {
        Ok(roll_message) => {
            discord.send_message(
                message.channel_id,
                &*format!("{} {}", message.author.mention().to_string(), roll_message),
                "",
                false,
            );
        }
        Err(_) => {}
    }
}

fn test(message: Message, discord: &Discord) {

}