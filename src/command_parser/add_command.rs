use super::*;

pub fn add(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context){

    let mut iter = msg.content.split(" ").filter(|word| word.len() >= 1);
    let _ = iter.next();



    if let Some(cmd) = iter.next(){
        
        let cmd_reply_iter = iter.clone();

        let mut cmd_reply = String::new();

        for word in cmd_reply_iter{
            &cmd_reply.push_str(&format!("{} ",word));
        }

        cmd_reply.pop();


        let iter_reply = cmd_reply.split(" ").filter(|word| word.len() >= 1);

        let mut cmd_reply_formattted  = String::new();

        for word in iter_reply{
            cmd_reply_formattted.push_str(&format!("{} ",word));
        }

        cmd_reply_formattted.pop();

        println!("cmd reply: {}",cmd_reply_formattted);



         
        if cmd_reply_formattted.len() >= 1{

             match env::var("FIREBASE_API_KEY"){
                 Ok(api_key)=> {
                     println!("checking for auth");

            
            let mut  auth_token = AUTH_TOKEN.lock();

            if let None = auth_token.as_ref() {
                match refresh_auth_token(&api_key){
                    Ok(token_new) => *auth_token = Some(token_new),
                    Err(err) => println!("ERROR while refreshing auth token: {}",err) 
                }
            }


            if let Err(why) =  add_custom_command_half_front_end(&msg, &ctx, cmd, &cmd_reply_formattted, auth_token.as_ref().expect("Error while reading auth token")){

                println!("Error adding custom command,{}\nTrying again with new auth token! ",why);

                match refresh_auth_token(&api_key){
                    Ok(token_new) => *auth_token = Some(token_new),
                    Err(err) => println!("ERROR while refreshing auth token: {}",err) 
                }

                if let Err(why) = add_custom_command_half_front_end(&msg, &ctx, cmd, &cmd_reply_formattted, auth_token.as_ref().expect("Error while reading auth token")){
                    println!("Error adding custom command: {}",why);

                    if let Err(why) = msg.channel_id.say(&ctx.http, format!("Error adding custom command: `{}`",why)) {
                        println!("Error sending message: {:?}", why);
                    };

                }

            
            }

   
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


    
    

}


fn add_custom_command_half_front_end(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context, cmd:&str, cmd_reply: &str,auth_token: &str) -> Result<(),String>{

    let resp = add_custom_command(cmd, &cmd_reply,auth_token);
        
    match resp{
        Ok(_) => {
            println!("Succesfully added custom command: {}",cmd);
            println!("Reply set to: {}",cmd_reply);
            if let Err(why) = msg.channel_id.say(&ctx.http, "Succesfully added custom command!") {
                println!("Error sending message: {:?}", why);
            }

            Ok(())
            
        },
        Err(why) => {

            println!("Error adding custom command. ERROR: {}",why);

           Err(why)

        }
    }

}



fn add_custom_command(command: &str,reply:&str, auth: &str) -> Result<(),String>{

    let resp = ureq::patch(&format!("https://plane-bot.firebaseio.com/commands.json?auth={}",auth)).send_json(ureq::json!({command:reply}));
    if resp.ok(){
        Ok(())
    }else{
        let status_code = resp.status();
        let response = &resp.into_string().unwrap_or_default();
        Err(format!("Error while sending PATCH request to database. Status code:  {}. Response: {}",status_code,response))
    }

}





