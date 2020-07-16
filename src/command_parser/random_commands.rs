
use super::*;

pub fn rand(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context){
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
             },
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





pub fn tail_or_head(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context){
    let head = random();

    if head{
        if let Err(why) = msg.channel_id.say(&ctx.http, "Head!") {
            println!("Error sending message: {:?}", why);
        }
    }else{
        if let Err(why) = msg.channel_id.say(&ctx.http, "Tail!") {
            println!("Error sending message: {:?}", why);
        };
    };
    
}

pub fn roll(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context){
    let mut rng = rand::thread_rng();

             if let Err(why) = msg.channel_id.say(&ctx.http, format!("You got {} !",rng.gen_range(1,7)) ) {
                println!("Error sending message: {:?}", why);
            };

}