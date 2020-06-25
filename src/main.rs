use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

#[group]
#[commands(ping, start_loop,sudo,ok,who)]

struct General;

use std::env;

struct Handler;

impl EventHandler for Handler {
}




fn main() {

    

    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("BOT_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("p!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {

    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
fn start_loop(ctx: &mut Context, msg: &Message) -> CommandResult{

   msg.channel_id.say(&ctx.http,"dont say this again").expect("cannot send message :(");

    Ok(())
}

#[command]
fn sudo(ctx: &mut Context, msg: &Message) -> CommandResult{

    msg.channel_id.say(&ctx.http,"this is not bash, keep that in mind").expect("cannot send message :(");
 
     Ok(())
 }

 #[command]
 fn ok(ctx: &mut Context, msg: &Message) -> CommandResult{
 
     msg.channel_id.say(&ctx.http,"Ok then ").expect("cannot send message :(");
  
      Ok(())
  }
 
  #[command]
 fn who(ctx: &mut Context, msg: &Message) -> CommandResult{
 
     msg.channel_id.say(&ctx.http,"I'm a plane bot! 🛩🛩🛩🛩🤖🤖🤖🤖").expect("cannot send message :(");
  
      Ok(())
  }
