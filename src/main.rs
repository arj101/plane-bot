
extern crate meval;
extern crate tiny_http;
extern crate serde_json;


mod command_parser;



use tiny_http::{Server, Response};


use std::{thread,env,fs};


use serenity::{
    model::{channel::{Message}, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
 
    fn message(&self, ctx: Context, msg: Message) {
        command_parser::parse(&msg,&ctx);
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





