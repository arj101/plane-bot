
extern crate meval;
extern crate tiny_http;

use tiny_http::{Server, Response};

use std::{
    env,
    fs,
    thread
};



use rand::{prelude::*,Rng};






use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
 
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "p!ping" {
    
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "p!hi" {
         
           
            if let Err(why) = msg.channel_id.say(&ctx.http, "hello!") {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "p!pong" {
  
            if let Err(why) = msg.channel_id.say(&ctx.http, "pinggggggggg!") {
                println!("Error sending message: {:?}", why);
            }
        }

        else if msg.content == "p!ok" {
        
            if let Err(why) = msg.channel_id.say(&ctx.http, "ok then") {
                println!("Error sending message: {:?}", why);
            }
        }

        else if msg.content == "p!help" {

            println!("Somebody is asking for help! 😃");

             if let Ok(text) = fs::read_to_string("./help.md"){
                println!("Succesfully read help.md 😄");
                if text.len() >= 1{
                    if let Err(why) = msg.channel_id.say(&ctx.http, text) {
                        println!("Error sending message: {:?}", why);
                    }
                }else {
                    println!("help.md is less than one character long! 🤔");
                    if let Err(why) = msg.channel_id.say(&ctx.http, "help.md is less than one character long! 🤔") {
                        println!("Error sending message: {:?}", why);
                    }
                }

            }else {
                println!("Error reading help.md! 🤔");
                if let Err(why) = msg.channel_id.say(&ctx.http, "Error reading help.md! 🤔\nFigure it out yourself 😉") {
                    println!("Error sending message: {:?}", why);
                }
            }
            

           
        }


        else if msg.content.starts_with("p!eval"){



            match meval::eval_str(crop_letters(&msg.content, 6)){
                Ok(res) => {
                    println!("{}",res);
                    if let Err(why) = msg.channel_id.say(&ctx.http,res) {
                                       println!("Error sending message: {:?}", why);
                    }
                },
                Err(why) => {
                    println!("error while parsing");
                    println!("{}",why);
                    if let Err(why) = msg.channel_id.say(&ctx.http,format!("Err: {}",why)) {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
           
        }

        else if msg.content == "p!bye" {
         
            if let Err(why) = msg.channel_id.say(&ctx.http, "byeeee!") {
                println!("Error sending message: {:?}", why);
            }
            if let Err(why) = msg.channel_id.say(&ctx.http, "👋") {
                println!("Error sending message: {:?}", why);
            }
        }

        else if msg.content == "p!tail or head" {

        	let head = random();

        	if head{
        		if let Err(why) = msg.channel_id.say(&ctx.http, "Head!") {
                println!("Error sending message: {:?}", why);
            }
        	}else{

        	if let Err(why) = msg.channel_id.say(&ctx.http, "Tail!") {
                println!("Error sending message: {:?}", why);
            }
           }

        }

         else if msg.content == "p!roll" {

         	let mut rng = rand::thread_rng();

         	if let Err(why) = msg.channel_id.say(&ctx.http, format!("You got {} !",rng.gen_range(1,7)) ) {
                println!("Error sending message: {:?}", why);
            }
        	
        }


         else if msg.content.starts_with("p!random") {

         	let mut no_err = true;

         	let mut iter = msg.content.split(" ").filter(|word| word.len() >= 1);
         	let _ = iter.next();
         	let num1:i32 =  match iter.next() {
         		Some(num) => match num.parse::<i32>(){
         			Ok(number) => number,
         			Err(err) => {
         				no_err = false;
         				println!("Error while parsing to i32", );
         				println!("Err: {}",err );
         				if let Err(why) = msg.channel_id.say(&ctx.http,"Error while parsing 32bit interger" ) {
               				println!("Error sending message: {:?}", why);
             			};
            		
         				-1i32
         			}
         		}
         		None => {
         			no_err = false;
         			println!("Error: Lower range not found");
         		
         				if let Err(why) = msg.channel_id.say(&ctx.http,"Lower range nor found" ) {
               				println!("Error sending message: {:?}", why);
             		    };
            			
         			-1i32
         		}
         	};

         	let num2:i32 =  match iter.next() {
         		Some(num) => match num.parse::<i32>(){
         			Ok(number) => number,
         			Err(err) => {
         				no_err = false;
         				println!("Error while parsing to i32");
         				if let Err(why) = msg.channel_id.say(&ctx.http,"Error while parsing 32bit interger" ) {
               				println!("Error sending message: {:?}", why);
             		    };
         				println!("Err: {}",err );
         				-1i32
         			}
         		},
         		None => {
         			no_err = false;
         			println!("Error: Higher range not found" );
  
         			if let Err(why) = msg.channel_id.say(&ctx.http,"Higher range nor found" ) {
               			println!("Error sending message: {:?}", why);
             		};
            			
         			-1i32
         		}
         	};

         	if no_err{


         	let mut rng = rand::thread_rng();

         	if let Err(why) = msg.channel_id.say(&ctx.http, format!("You got {} !",rng.gen_range(num1,num2)) ) {
                println!("Error sending message: {:?}", why);
              };
            };
            	
        }

        else if msg.content.starts_with("p!"){
            if let Err(why) = msg.channel_id.say(&ctx.http, "This command doesn't exist, yet! ¯\\_(ツ)_/¯") {
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

    thread::spawn(
        ||{

            let port = match env::var("PORT"){
                Ok(port) => {
                    println!("Found env var PORT : {}",port);
                    port
                },
                Err(why) => {
                    println!("Error: {}",why);
                    println!("Defaulting to port 3000");
                    String::from("3000")
                }
            };

            let server = Server::http(format!("0.0.0.0:{}",port)).unwrap();

            for request in server.incoming_requests() {
                println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                    request.method(),
                    request.url(),
                    request.headers()
                );

                let response = Response::from_string("Thanks for waking me up!");
                if let Err(why) = &request.respond(response){
                    println!("Error sending response to http request : {}",why);
                }
            }
        }
    );
   
    let token = env::var("BOT_TOKEN")
    .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}


fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}
