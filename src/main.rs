
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

    	let prefix = "p!";
    	let eval_command: &str = &(prefix.to_owned() + "eval");
        let rand_command: &str = &(prefix.to_owned() + "random");
        let search_command: &str = &(prefix.to_owned() + "search");

        if msg.content == format!("{}ping",prefix ) {
    
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content ==  format!("{}hi",prefix )  {
         
           
            if let Err(why) = msg.channel_id.say(&ctx.http, "hello!") {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content ==  format!("{}pong",prefix )  {
  
            if let Err(why) = msg.channel_id.say(&ctx.http, "pinggggggggg!") {
                println!("Error sending message: {:?}", why);
            }
        }

        else if msg.content ==  format!("{}ok",prefix )  {
        
            if let Err(why) = msg.channel_id.say(&ctx.http, "ok then") {
                println!("Error sending message: {:?}", why);
            }
        }

        else if msg.content ==  format!("{}help",prefix )  {



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
       
 



        else if msg.content.starts_with(eval_command){



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

        else if msg.content == format!("{}bye",prefix ) {
         
            if let Err(why) = msg.channel_id.say(&ctx.http, "byeeee!") {
                println!("Error sending message: {:?}", why);
            }
            if let Err(why) = msg.channel_id.say(&ctx.http, "👋") {
                println!("Error sending message: {:?}", why);
            }
        }

        else if msg.content == format!("{}tail or head",prefix ) {

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

         else if msg.content == format!("{}roll",prefix ) {

         	let mut rng = rand::thread_rng();

         	if let Err(why) = msg.channel_id.say(&ctx.http, format!("You got {} !",rng.gen_range(1,7)) ) {
                println!("Error sending message: {:?}", why);
            }
        	
        }

    



         else if msg.content.starts_with(rand_command) {

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

        else if msg.content.starts_with(search_command){

            let mut iter = msg.content.split("\"").filter(|word| word.len() >= 1);
            let _ = iter.next();
            let keyword = if let Some(key) = iter.next(){
                                key
                            }else {
                                "Airbus A350"
                            };

            let mut keyword_new = String::new();

            for word in keyword.split(" ").filter(|word| word.len() >= 1){
                keyword_new.push_str(word);
                keyword_new.push_str("+");
            }

                        

            let mut search_engine:String = if let Some(engine) = iter.next(){
                                    engine.to_lowercase()
                                }else{
                                    String::from("duckduckgo")
                                };

            search_engine.retain(|c|  c !=' ');


            let  search_engine = search_engine.as_str();

        



            println!("{} {}",keyword,search_engine);
                
           match search_engine {
               "duckduckgo" => {
                    if let Err(why) = msg.channel_id.say(&ctx.http,format!("https://duckduckgo.com/?q={}",keyword_new)) {
                        println!("Error sending message: {:?}", why);
                    }
                },

                "bing" => {
                    if let Err(why) = msg.channel_id.say(&ctx.http,format!("https://www.bing.com/search?q={}",keyword_new)) {
                        println!("Error sending message: {:?}", why);
                    }
                },
                "google" => {
                    if let Err(why) = msg.channel_id.say(&ctx.http,format!("https://google.com/search?q={}",keyword_new)) {
                        println!("Error sending message: {:?}", why);
                    }
                },

                "wikipedia" => {
                    if let Err(why) = msg.channel_id.say(&ctx.http,format!("https://en.wikipedia.org/wiki/{}",keyword_new)) {
                        println!("Error sending message: {:?}", why);
                    }
                },

                _=> {
                    if let Err(why) = msg.channel_id.say(&ctx.http,format!("https://duckduckgo.com/?q={}",keyword_new)) {
                        println!("Error sending message: {:?}", why);
                    }
                }
           }
            
            


        }

        else if msg.content.starts_with(prefix){
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
