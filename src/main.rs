mod roll;

use std::env;
use discord::Discord;
use discord::model::Event;

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
                match roll::roll(&message.content) {
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
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Receive error: {:?}", err),
        }
    }
}

