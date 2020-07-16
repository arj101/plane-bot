
extern crate meval;
extern crate tiny_http;
extern crate serde_json;




use tiny_http::{Server, Response};
use rand::{prelude::*,Rng};
use std::{thread,env,fs,sync::Arc};


use serenity::{
    model::{channel::{Message}, gateway::Ready},
    prelude::*,
};


mod command_parser;

use lazy_static::lazy_static;


struct Handler;

impl EventHandler for Handler {
 
    fn message(&self, ctx: Context, msg: Message) {
        command_parser::parse(&msg,&ctx);
    }

   
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

lazy_static! {
    static ref AUTH_TOKEN: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
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




//copy pasted from StackOverFlow 😁
fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}


fn refresh_auth_token(api_key: &str) -> Result<String,String>{


    println!("refreshing auth token....");




    let id_token_resp = ureq::post(&format!("https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",api_key)).set("Content-Type", "application/json").send_json(ureq::json!({"returnSecureToken":true}));


    if id_token_resp.ok(){

        let response = &id_token_resp.into_string().unwrap_or_default();

        let resp_json: serde_json::Value = serde_json::from_str(response).unwrap_or_default();

        match resp_json.get("idToken"){
            Some(id) => {

                if let Some(id) = id.as_str(){
                    Ok(String::from(id))
                }else {
                    Err(String::from("diode"))
                }
            }
            None => {
                println!("Response from server didnt contain ID_TOKEN");
                Err(String::from("Response from server didnt contain ID_TOKEN"))
                
            }
        }


    }else {
        println!("Error in new token request, server didnt respod with OK");
        return Err(format!("Error in new token request, server didnt respod with OK"))
    }


}