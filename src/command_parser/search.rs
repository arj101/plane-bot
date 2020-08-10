pub fn search(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context) {
    let mut iter = msg
        .content
        .split("\"")
        .flat_map(|message| message.split("“"))
        .flat_map(|message| message.split("\n"))
        .flat_map(|message| message.split("”"))
        .filter(|word| word.len() >= 1);

    let _ = iter.next();
    let keyword = if let Some(key) = iter.next() {
        key
    } else {
        "Airbus A350"
    };

    let mut search_engine: String = if let Some(engine) = iter.next() {
        engine.to_lowercase()
    } else {
        String::from("duckduckgo")
    };

    search_engine.retain(|c| c != ' ');

    let search_engine = search_engine.as_str();

    println!("{} {}", keyword, search_engine);

    match search_engine {
        "duckduckgo" => {
            let keyword = keyword.replace(" ", "+");
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("https://duckduckgo.com/?q={}", keyword))
            {
                println!("Error sending message: {:?}", why);
            }
        }

        "bing" => {
            let keyword = keyword.replace(" ", "+");
            if let Err(why) = msg.channel_id.say(
                &ctx.http,
                format!("https://www.bing.com/search?q={}", keyword),
            ) {
                println!("Error sending message: {:?}", why);
            }
        }
        "google" => {
            let keyword = keyword.replace(" ", "+");
            if let Err(why) = msg.channel_id.say(
                &ctx.http,
                format!("https://google.com/search?q={}", keyword),
            ) {
                println!("Error sending message: {:?}", why);
            }
        }

        "wikipedia" => {
            let keyword = keyword.replace(" ", "_");
            if let Err(why) = msg.channel_id.say(
                &ctx.http,
                format!("https://en.wikipedia.org/wiki/{}", keyword),
            ) {
                println!("Error sending message: {:?}", why);
            }
        }

        _ => {
            let keyword = keyword.replace(" ", "+");
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("https://duckduckgo.com/?q={}", keyword))
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
