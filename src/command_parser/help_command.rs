use std::fs;

pub fn help(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context){
    println!("Somebody is asking for help! 😃");

    if let Ok(text) = fs::read_to_string("./commands/help.md"){
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
}