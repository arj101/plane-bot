
extern crate meval;
extern crate tiny_http;
extern crate serde_json;

mod search;

use tiny_http::{Server, Response};


use std::{
    env,
    fs,
    thread,
};



use rand::{prelude::*,Rng};






use serenity::{
    model::{channel::{Message}, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
 
    fn message(&self, ctx: Context, msg: Message) {
       

        let mut cmd_found = true;

        let command = match msg.content.split(" ").flat_map(|word| word.split("\n")).filter(|word| word.len() >= 1).next(){
            Some(cmd) => cmd,
            None => {
                cmd_found = false;
                "a781gh487c892c2ubgy"
            }

        };



        let prefix                 = "p!";

        let _ping_command          = format!("{}ping",prefix);
        let _hi_command            = format!("{}hi",prefix);
        let _pong_command          = format!("{}pong",prefix);
        let _ok_command            = format!("{}ok",prefix);
        let _help_command          = format!("{}help",prefix);
        let _eval_command          = format!("{}eval",prefix);
        let _bye_command           = format!("{}bye",prefix);
        let _tail_or_head_command  = format!("{}tail or head",prefix);
        let _roll_command          = format!("{}roll",prefix);
        let _random_command        = format!("{}random",prefix);
        let _search_command        = format!("{}search",prefix);
        let _test_command          = format!("{}test",prefix);
        let _add_command_command   = format!("{}addCommand",prefix);
 



        match command{

            _ if  _ping_command == command => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                    println!("Error sending message: {:?}", why);
                };

                
                
            },

            _ if  _add_command_command == command => {


                let mut iter = msg.content.split(" ").filter(|word| word.len() >= 1);
                let _ = iter.next();

                if let Some(cmd) = iter.next(){
                    
                     
                    if let Some(reply) = iter.next(){

                         match env::var("FIREBASE_ACCESS_TOKEN"){
                             Ok(token)=> {

                        let resp = ureq::patch(&format!("https://plane-bot.firebaseio.com/commands.json?access_token={}",token)).send_json(ureq::json!({cmd:reply}));

                        if resp.ok() {
                            println!("Succesfully added custom command: {}",cmd);
                            println!("Reply set to: {}",reply);
                            if let Err(why) = msg.channel_id.say(&ctx.http, "Succesfully added custom command!") {
                                println!("Error sending message: {:?}", why);
                            };
            
                        } else {

                            let status_code = resp.status();

                            let response = &resp.into_string().unwrap_or_default();

                            println!("Error adding custom command: {}: {}",status_code,response);
                            if let Err(why) = msg.channel_id.say(&ctx.http, format!("Error adding custom command. Status code:{} , Response: {}",status_code, response)) {
                                println!("Error sending message: {:?}", why);
                            };
                        };
                    },
                    Err(why) => {
                        println!("Error while looking up firebase access token: {:?}",why);
                        if let Err(why) = msg.channel_id.say(&ctx.http, "Cannot find firebase access token in environment variables.") {
                            println!("Error sending message: {:?}", why);
                        };
                    }
                }
                    }else{
                        println!("Reply for command to add not found!");
                        if let Err(why) = msg.channel_id.say(&ctx.http,"Reply for command to add not found!") {
                            println!("Error sending message: {:?}", why);
                        };
                    }
                    
                }else {
                    println!("Command to add not found!");
                    if let Err(why) = msg.channel_id.say(&ctx.http,"Command to add not found!") {
                        println!("Error sending message: {:?}", why);
                    };
                }


                
                
                
            },


            _ if _hi_command == command => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "hello!") {
                    println!("Error sending message: {:?}", why);
                };

                
            },


            _ if _pong_command == command => {

                if let Err(why) = msg.channel_id.say(&ctx.http, "pinggggggggg!") {
                    println!("Error sending message: {:?}", why);
                };

                
            },

            _ if _ok_command == command => {

                if let Err(why) = msg.channel_id.say(&ctx.http, "ok then") {
                    println!("Error sending message: {:?}", why);
                };

                
            },

            _ if _help_command == command => {

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
                };

                }else {
                    println!("Error reading help.md! 🤔");
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Error reading help.md! 🤔\nFigure it out yourself 😉") {
                        println!("Error sending message: {:?}", why);
                    }
                };

                
            },


            _ if _eval_command == command => {
                match meval::eval_str(crop_letters(&msg.content, 6)){
                    Ok(res) => {
                        println!("{}",res);
                        if let Err(why) = msg.channel_id.say(&ctx.http,res) {
                            println!("Error sending message: {:?}", why);
                        };
                    },
                    Err(why) => {
                        println!("error while parsing");
                        println!("{}",why);
                        if let Err(why) = msg.channel_id.say(&ctx.http,format!("Err: {}",why)) {
                            println!("Error sending message: {:?}", why);
                        };
                    }
                };

                
            },


            _ if _bye_command == command => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "byeeee!") {
                    println!("Error sending message: {:?}", why);
                };
                if let Err(why) = msg.channel_id.say(&ctx.http, "👋") {
                    println!("Error sending message: {:?}", why);
                };

                
            },


            _ if _tail_or_head_command == command => {
                let head = random();

                if head{
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Head!") {
                        println!("Error sending message: {:?}", why);
                    }
                }else{
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Tail!") {
                        println!("Error sending message: {:?}", why);
                    };
                };
                
            },

            _ if _roll_command == command=>  {
                let mut rng = rand::thread_rng();

         	    if let Err(why) = msg.channel_id.say(&ctx.http, format!("You got {} !",rng.gen_range(1,7)) ) {
                    println!("Error sending message: {:?}", why);
                };

                
            },


            _ if _random_command == command => {
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
         		},
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
            
            
        },

        _ if _search_command == command=> {


            search::search(&msg, &ctx);

           
        },


        _ if _test_command == command=> {

        

        
            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("Hello, World!");
                m.embed(|e| {
                    e.title("This is a title");
                    e.description("This is a description");
                    e.image("attachment://screenshot.png");
                    e.fields(vec![
                        ("This is the first field", "This is a field body", true),
                        ("This is the second field", "Both of these fields are inline", true),
                    ]);
                    e.field("This is the third", "This is not an inline field", false);
                    e.footer(|f| {
                        f.text("This is a footer");

                        f
                    });

                    e
                });
                m
            });

            if let Err(why) = msg {
                println!("Error sending message: {:?}", why);
            }

            

        },
              
            
            _ => {


                cmd_found = false;
                
            }
        };


        if msg.content.starts_with(prefix) && !cmd_found{

            println!("Command not found in code, starting to lookup database");

            let command_without_prefix = crop_letters(&msg.content, 2);

            println!("command without prefix: {}",command_without_prefix);

            let resp = ureq::get("https://plane-bot.firebaseio.com/commands.json")
            .send_string("");


            let response:String;



            if resp.ok() {

                response = resp.into_string().unwrap_or_default();
                println!("GET request to firebase database succesful!");

            } else {

                println!("Error sending GET request to firebase database: {}: {}", resp.status(), resp.into_string().unwrap_or_default());
                response = String::from("Error sending GET request to firebase database");

            }


            let v:serde_json::Value = if let Ok(cmd_and_resps) = serde_json::from_str(&response){
                cmd_and_resps
            }else {
                serde_json::from_str("parse error while converting to serde::json from str").unwrap_or_default()
            };

            if let Some(cmd_resp) = v.get(command_without_prefix){

                let mut cmd_resp = format!("{}",cmd_resp);

                cmd_resp.pop();
                cmd_resp = crop_letters(&cmd_resp, 1).to_string();

                let mut iter = cmd_resp.split("&%nm%").filter(|each_message| each_message.len() >= 1);

                for message in iter{
                    if let Err(why) = msg.channel_id.say(&ctx.http,message ) {
                        println!("Error sending message: {:?}", why);
                    }
                }

                println!("Response for command {} found in database: {}",command_without_prefix,cmd_resp);

              

                cmd_found = true;

            }else {
                println!("Command {} not found in firebase database",command_without_prefix);
                cmd_found = false;
            }

        }



        if msg.content.starts_with(prefix) && !cmd_found{
            if let Err(why) = msg.channel_id.say(&ctx.http, "This command doesn't exist, yet! ¯\\_(ツ)_/¯") {
                println!("Error sending message: {:?}", why);
            }
        };


    }

   
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


fn main() {

    thread::spawn(
        ||{

            // let mut ip:&str;

            let port = match env::var("PORT"){
                Ok(port) => {
                    println!("Found env var PORT : {}",port);
                    // ip = "0.0.0.0";
                    port
                },
                Err(why) => {
                    println!("Error: {}",why);
                    println!("Defaulting to port 3000");
                    // ip="127.0.0.0";
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

                let wakeup_content = if let Ok(read_file) =  fs::read_to_string("./public/wakeup.html"){
                	read_file
                }else {
                	String::from("WATCHING! 👀")
                };

                let mut response = Response::from_string(wakeup_content);
                let content_type_header = tiny_http::Header::from_bytes(&b"Content-Type"[..],&b"text/html"[..]).expect("Failed creating header");
                response.add_header(content_type_header);
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
