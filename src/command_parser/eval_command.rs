

pub fn eval(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context){
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

}
fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}