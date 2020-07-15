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
            
            let id_token_resp = ureq::post(&format!("https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",api_key)).set("Content-Type", "application/json").send_json(ureq::json!({"returnSecureToken":true}));


            if id_token_resp.ok(){

                let response = &id_token_resp.into_string().unwrap_or_default();

                let resp_json: serde_json::Value = serde_json::from_str(response).unwrap_or_default();

                let  id = String::from(resp_json.get("idToken").unwrap().as_str().unwrap());


            let resp = ureq::patch(&format!("https://plane-bot.firebaseio.com/commands.json?auth={}",id)).send_json(ureq::json!({cmd:cmd_reply_formattted}));

            if resp.ok() {

                println!("Succesfully added custom command: {}",cmd);
                println!("Reply set to: {}",cmd_reply_formattted);
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