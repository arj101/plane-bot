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

extern crate tiny_http;

use std::sync::Arc;
use std::thread;

#[group]
#[commands(ping, start_loop,sudo,ok,who)]

struct General;

use std::env;

struct Handler;

impl EventHandler for Handler {
}




fn main() {

    thread::spawn(|| {

    let port = if let Ok(num) = env::var("PORT"){
                println!("found env var PORT, setting port to {}",num);
                    num
                }else {
                    println!("cannot find env var PORT, defaulting to 3000");
                    String::from("3000")
                };

    let server = Arc::new(tiny_http::Server::http(format!("127.0.0.1:{}",port)).unwrap());
    println!("Now listening on port {}",port);

    let mut handles = Vec::new();

    for _ in 0 .. 4 {
        let server = server.clone();

        handles.push(thread::spawn(move || {
            for rq in server.incoming_requests() {
                let response = tiny_http::Response::from_string("watching...👀".to_string());
                let _ = rq.respond(response);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    });

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
