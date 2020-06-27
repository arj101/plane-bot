
extern crate tiny_http;

use std::sync::Arc;
use std::thread;


use std::env;



use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "p!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "p!hi" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "hello!") {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "p!pong" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "pinggggggggg!") {
                println!("Error sending message: {:?}", why);
            }
        }

        else if msg.content == "p!ok" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "ok then") {
                println!("Error sending message: {:?}", why);
            }
        }

        else if msg.content == "p!help" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "figure out yourself😉") {
                println!("Error sending message: {:?}", why);
            }
        }

        else if msg.content.starts_with("p!eval"){

            let mut iter  = msg.content.split(" ").filter(|word| word.len() >= 1);
            let _ = iter.next();

            let mut no_err = true;

            let a = if let Ok(num) = iter.next().unwrap_or_default().parse::<f64>(){
                num
            }else{
                if let Err(why) = msg.channel_id.say(&ctx.http, "thats not valid 64bit float, i guess :/") {
                    println!("Error sending message: {:?}", why);
                };
                no_err = false;
                0.0f64
            };

            if no_err{
            
        
            let symbol = iter.next().unwrap_or_default();

            let b = if let Ok(num) = iter.next().unwrap_or_default().parse::<f64>(){
                num
            }else{
                if let Err(why) = msg.channel_id.say(&ctx.http, "thats not valid 64bit float, i guess :/") {
                    println!("Error sending message: {:?}", why);
                };
                no_err = false;
                0.0f64
            };
            

            println!("{} {} {}",a,symbol,b);
            
            if no_err{
            match symbol{
                "+" => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, format!("Its {} !",a+b)) {
                        println!("Error sending message: {:?}", why);
                    }
                }

                "-" => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, format!("Its {} !",a-b)) {
                        println!("Error sending message: {:?}", why);
                    }
                }
                "/" => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, format!("Its {} !",a/b)) {
                        println!("Error sending message: {:?}", why);
                    }
                }
                "*" => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, format!("Its {} !",a*b)) {
                        println!("Error sending message: {:?}", why);
                    }
                }
                _ => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "idk what to do!") {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }
        }

           
        }

        else if msg.content == "p!bye" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "byeeee!") {
                println!("Error sending message: {:?}", why);
            }
            if let Err(why) = msg.channel_id.say(&ctx.http, "👋") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
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

    let token = env::var("BOT_TOKEN")
    .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}