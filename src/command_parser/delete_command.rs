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


                        let mut auth_token = AUTH_TOKEN.lock();


                        

                        if let Some(cmd) = cmd{

                            if let None = auth_token.as_ref() {
                                match refresh_auth_token(&key){
                                    Ok(token_new) => *auth_token = Some(token_new),
                                    Err(err) => println!("ERROR while refreshing auth token: {}",err) 
                                }
                            }

                            if let Err(why) = delete_custom_command_half_front_end(&msg, &ctx, cmd, auth_token.as_ref().expect("Error while reading AUTH_TOKEN")){
                                println!("Error deleting custom command,{}\nTrying again with new auth token! ",why);

                                match refresh_auth_token(&key){
                                    Ok(token_new) => *auth_token = Some(token_new),
                                    Err(err) => println!("ERROR while refreshing auth token: {}",err) 
                                }

                                if let Err(why) = delete_custom_command_half_front_end(&msg, &ctx, cmd, auth_token.as_ref().expect("Error while reading AUTH_TOKEN")){
                                    println!("Error deleting custom command: {}",why);

                                    if let Err(why) = msg.channel_id.say(&ctx.http, format!("Error deleting custom command: `{}`",why)) {
                                        println!("Error sending message: {:?}", why);
                                    };
                                }

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


fn delete_custom_command(command: &str, auth: &str) -> Result<(),String>{

    let resp = ureq::patch(&format!("https://plane-bot.firebaseio.com/commands/{}.json?auth={}",command,auth)).send_json(ureq::json!(serde_json::Value::Null));
    if resp.ok(){
        Ok(())
    }else{
        let status_code = resp.status();
        let response = &resp.into_string().unwrap_or_default();
        Err(format!("Error while sending PATCH request to database. Status code:  {}. Response: {}",status_code,response))
    }

}


fn delete_custom_command_half_front_end(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context, cmd:&str ,auth_token: &str) -> Result<(),String>{

    let resp = delete_custom_command(cmd,auth_token);
        
    match resp{
        Ok(_) => {
            println!("Succesfully deleted custom command: `{}` !",cmd);
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("Succesfully deleted custom command: `{}` !",cmd)) {
                println!("Error sending message: {:?}", why);
            }

            Ok(())
            
        },
        Err(why) => {

            println!("Error deleting custom command. ERROR: {}",why);

            Err(why)

        }
    }

}


fn member_is_admin(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context) -> Result<bool,String>{

   

    if msg.author.id.0 == 685093043078037534{//checking if its me, 😁
        return Ok(true)
    }


    match &msg.member{
        Some(_) => {

            match &ctx.http.get_guild(msg.guild_id.unwrap_or_default().0){
                Ok(guild_partial) => {

                    let mut has_perm = false;


                    let mut err = false;

                    for (_key, value) in &guild_partial.roles{

                        if value.permissions.administrator(){
                            if let Ok(admin_perm) = msg.author.has_role(&ctx.http, guild_partial.id,value ){
                               has_perm = admin_perm;
                            }else{
                                err = true;
                            }
                        }
  
                    }

                    if err{
                       return  Err(String::from("Error while checking user has admin roles or not"))
                    }else{
                       Ok(has_perm)
                    }
  
                },
                Err(_) => Err(String::from("Error getting Partial Guild"))
            }

        },
        None => Err(String::from("Cannot find member property for message"))
    }


   
}