mod gods;
mod scraper;

use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult, StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

fn main() -> Result<(), isahc::Error> {
    let token = env::var("DISCORD_TOKEN").expect("Expected bot token in env");
    let mut client = Client::new(&token, Handler).expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("~"))
            .group(&GENERAL_GROUP),
    );

    if let Err(e) = client.start() {
        panic!("Bot Error: {:?}", e);
    }

    let items = scraper::get_god_build("khepri").unwrap();
    println!("{:?}", items[0]);
    println!("{:?}", items[1]);

    Ok(())
}

#[group]
#[commands(build, update)]
struct General;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
    }
}

#[command]
fn build(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let god_name = args.rest();
    println!("{}", god_name);

    if let Ok(mut e) = scraper::get_god_build(god_name) {
        let start_tag = String::from("---------\nStarter\n---------\n");
        let ending = e.pop().unwrap().join("\n");
        let end_tag = String::from("\n----------\nEnding\n----------\n");
        let starter = e.pop().unwrap().join("\n");
        let build = start_tag + &starter + &end_tag + &ending;

        if let Err(why) = message.channel_id.say(&context.http, build) {
            println!("Error sending message: {:?}", why);
        }
    }

    Ok(())
}

#[command]
fn update(context: &mut Context, message: &Message) -> CommandResult {
    let names = gods::read_in_names();
    if let Err(e) = gods::delete_all_html(names.clone()) {
        println!("Error deleting old html: {:?}", e);
        if let Err(why) = message
            .channel_id
            .say(&context.http, "Error deleting old HTML data")
        {
            println!("Error sending message: {:?}", why);
        }
    }

    if let Err(e) = gods::download_all_html(names) {
        println!("Error download html: {:?}", e);
        if let Err(why) = message
            .channel_id
            .say(&context.http, "Error downloading HTML data")
        {
            println!("Error sending message: {:?}", why);
        }
    }

    if let Err(why) = message.channel_id.say(&context.http, "Data updated") {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
