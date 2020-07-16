use super::*;


pub fn delete(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context){
    let member = &msg.member;

            println!("Got custom command delete request from : {}",msg.author.name);

            match member{
                Some(_) => {

                  let mut has_perm = false;

                  if msg.author.id.0 == 685093043078037534{
                      has_perm = true;
                  }

                  let  guild = &ctx.http.get_guild(msg.guild_id.unwrap_or_default().0).unwrap();


                  for (_key, value) in &guild.roles{

                      if value.permissions.administrator(){
                          if msg.author.has_role(&ctx.http, guild.id,value ).expect(":|"){
                              has_perm = true;
                          }
                      };

                  }
           
                  if has_perm{
                    println!("user is admin");
                   

                    if let Ok(key) = env::var("FIREBASE_API_KEY"){

                        let mut iter = msg.content.split(" ").filter(|word| word.len() >= 1);
                        let _ = iter.next();
                        let cmd = iter.next();

                        if let Some(cmd) = cmd{

                            let id_token = ureq::post(&format!("https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",key)).set("Content-Type", "application/json").send_json(ureq::json!({"returnSecureToken":true}));

                            if id_token.ok(){


                                let response = id_token.into_string().unwrap_or_default();
                                let response_json: serde_json::Value = serde_json::from_str(&response).unwrap_or_default();

                              
                                match response_json.get("idToken"){
                                    Some(id_token) => {
                                        if let Some(id_token) = id_token.as_str(){
                                            let resp = ureq::patch(&format!("https://plane-bot.firebaseio.com/commands/{}.json?auth={}",cmd,id_token)).send_json(ureq::json!(serde_json::Value::Null));

                                            if resp.ok(){
    
        
        
                                                println!("Succesfully deleted command: `{}`",cmd);
        
                                                if let Err(why) = msg.channel_id.say(&ctx.http, format!("Succesfully deleted command: `{}`",cmd)) {
                                                    println!("Error sending message: {:?}", why);
                                                };
        
        
                                            }else {
                                                let status_code = resp.status();
        
                                                let response = resp.into_string().unwrap_or_default();
        
                                                println!("Error while sending DELETE request to database. Status code: {:?}, Response: {:?}",status_code,response);
                                                if let Err(why) = msg.channel_id.say(&ctx.http, format!("Error while sending DELETE request to database. Status code: {:?}, Response: {:?}",status_code,response)) {
                                                    println!("Error sending message: {:?}", why);
                                                };
                                            };

                                        }else{}
                                    },
                                    None => {}
                                };




                            }else{
                                let status_code = id_token.status();

                                let response = id_token.into_string().unwrap_or_default();

                                println!("Error while requesting id_token to firebase. Status code: {:?}, Response: {:?}",status_code,response);
                                if let Err(why) = msg.channel_id.say(&ctx.http, format!("Error while requesting id_token to firebase. Status code: {:?}, Response: {:?}",status_code,response)) {
                                    println!("Error sending message: {:?}", why);
                                };
                            }


                        }else{
                            println!("Command to remove not found");
                            if let Err(why) = msg.channel_id.say(&ctx.http, "Command to remove not found") {
                                println!("Error sending message: {:?}", why);
                            };
                        }

                    }else{
                        println!("api key not found in env");
                        if let Err(why) = msg.channel_id.say(&ctx.http, "api key not found in environment variables") {
                            println!("Error sending message: {:?}", why);
                        };
                    }

                  

                  }else {
                   
                    println!("user is not admin");
                    if let Err(why) = msg.channel_id.say(&ctx.http, "You dont have permission to delete a custom command!") {
                        println!("Error sending message: {:?}", why);
                    };
                  }

                },
                None => {}
            };

}