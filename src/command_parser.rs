


mod search;

mod add_command;
mod delete_command;
mod help_command;
mod eval_command;
mod random_commands;
use super::*;




pub fn parse(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context){
     

    let mut cmd_found = true;

    let command = match msg.content.split(" ").flat_map(|word| word.split("\n")).filter(|word| word.len() >= 1).next(){
        Some(cmd) => cmd,
        None => {
            cmd_found = false;
            "__command__not__found___"
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
    let _tail_or_head_command  = format!("{}tailOrHead",prefix);
    let _roll_command          = format!("{}roll",prefix);
    let _random_command        = format!("{}random",prefix);
    let _search_command        = format!("{}search",prefix);
    let _test_command          = format!("{}test",prefix);  
    let _add_command_command   = format!("{}addCommand",prefix);//not a typo
    let _delete_command_command = format!("{}deleteCommand",prefix);//not a typo




    match command{

        _ if  _ping_command == command => {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                println!("Error sending message: {:?}", why);
            };

            
            
        },

        _ if _delete_command_command == command => {
            delete_command::delete(&msg,&ctx);
        },


        _ if  _add_command_command == command => {
            add_command::add(&msg,&ctx);
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
            help_command::help(&msg,&ctx);
        },


        _ if _eval_command == command => {
           eval_command::eval(&msg,&ctx);
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
          random_commands::tail_or_head(&msg,&ctx);
        },

        _ if _roll_command == command=>  {
            random_commands::roll(&msg,&ctx);
        },


        _ if _random_command == command => {
            random_commands::rand(&msg,&ctx);
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

            let iter = cmd_resp.split("&%nm%").filter(|each_message| each_message.len() >= 1);

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

