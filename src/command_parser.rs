mod search;

mod add_command;
mod delete_command;
mod eval_command;
mod help_command;
mod random_commands;
mod run_command;
use super::*;

use std::thread;

pub fn parse(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context) {
    let mut cmd_found = true;

    let command = match msg
        .content
        .split(" ")
        .flat_map(|word| word.split("\n"))
        .filter(|word| word.len() >= 1)
        .next()
    {
        Some(cmd) => cmd,
        None => {
            cmd_found = false;
            "__command__not__found___"
        }
    };

    macro_rules! prefix {
        () => {
            "p!"
        };
    }

    mod commmands {
        pub const PING: &str = concat!(prefix!(), "ping");
        pub const HI: &str = concat!(prefix!(), "hi");
        pub const PONG: &str = concat!(prefix!(), "pong");
        pub const OK: &str = concat!(prefix!(), "ok");
        pub const HELP: &str = concat!(prefix!(), "help");
        pub const EVAL: &str = concat!(prefix!(), "eval");
        pub const BYE: &str = concat!(prefix!(), "bye");
        pub const TAIL_OR_HEAD: &str = concat!(prefix!(), "tailOrHead");
        pub const ROLL: &str = concat!(prefix!(), "roll");
        pub const RANDOM: &str = concat!(prefix!(), "random");
        pub const SEARCH: &str = concat!(prefix!(), "search");
        pub const TEST: &str = concat!(prefix!(), "test");
        pub const ADD_COMMAND: &str = concat!(prefix!(), "addCommand");
        pub const DELETE_COMMAND: &str = concat!(prefix!(), "deleteCommand");
        pub const RANDOM_NUM: &str = concat!(prefix!(), "randNum");
        pub const RUN: &str = concat!(prefix!(), "run");
    }

    match command {
        commmands::RUN => {
            let _ = run_command::run(&msg, &ctx);
        }

        commmands::PING => {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                println!("Error sending message: {:?}", why);
            };
        }

        commmands::DELETE_COMMAND => {
            delete_command::delete(&msg, &ctx);
        }

        commmands::ADD_COMMAND => {
            add_command::add(&msg, &ctx);
        }

        commmands::HI => {
            if let Err(why) = msg.channel_id.say(&ctx.http, "hello!") {
                println!("Error sending message: {:?}", why);
            };
        }

        commmands::PONG => {
            if let Err(why) = msg.channel_id.say(&ctx.http, "pinggggggggg!") {
                println!("Error sending message: {:?}", why);
            };
        }

        commmands::OK => {
            if let Err(why) = msg.channel_id.say(&ctx.http, "ok then") {
                println!("Error sending message: {:?}", why);
            };
        }

        commmands::HELP => {
            help_command::help(&msg, &ctx);
        }

        commmands::EVAL => {
            eval_command::eval(&msg, &ctx);
        }

        commmands::BYE => {
            if let Err(why) = msg.channel_id.say(&ctx.http, "byeeee!") {
                println!("Error sending message: {:?}", why);
            };
            if let Err(why) = msg.channel_id.say(&ctx.http, "👋") {
                println!("Error sending message: {:?}", why);
            };
        }

        commmands::TAIL_OR_HEAD => {
            random_commands::tail_or_head(&msg, &ctx);
        }

        commmands::ROLL => {
            random_commands::roll(&msg, &ctx);
        }

        commmands::RANDOM => {
            random_commands::rand(&msg, &ctx);
        }

        commmands::SEARCH => {
            search::search(&msg, &ctx);
        }

        commmands::RANDOM_NUM => {
            random_commands::random_num(&msg, &ctx);
        }

        commmands::TEST => {
            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("Hello, World!");
                m.embed(|e| {
                    e.title("This is a title");
                    e.description("This is a description");
                    e.image("attachment://screenshot.png");
                    e.fields(vec![
                        ("This is the first field", "This is a field body", true),
                        (
                            "This is the second field",
                            "Both of these fields are inline",
                            true,
                        ),
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
        }

        _ => {
            cmd_found = false;
        }
    };

    if msg.content.starts_with(prefix!()) && !cmd_found {
        println!("Command not found in code, starting to lookup database");

        let command_without_prefix = crop_letters(&msg.content, 2);

        println!("command without prefix: {}", command_without_prefix);

        let resp = ureq::get("https://plane-bot.firebaseio.com/commands.json").send_string("");

        let response: String;

        if resp.ok() {
            response = resp.into_string().unwrap_or_default();
            println!("GET request to firebase database succesful!");
        } else {
            println!(
                "Error sending GET request to firebase database: {}: {}",
                resp.status(),
                resp.into_string().unwrap_or_default()
            );
            response = String::from("Error sending GET request to firebase database");
        }

        let v: serde_json::Value = if let Ok(cmd_and_resps) = serde_json::from_str(&response) {
            cmd_and_resps
        } else {
            serde_json::from_str("parse error while converting to serde::json from str")
                .unwrap_or_default()
        };

        if let Some(cmd_resp) = v.get(command_without_prefix) {
            let mut cmd_resp = format!("{}", cmd_resp);

            cmd_resp.pop();
            cmd_resp = crop_letters(&cmd_resp, 1).to_string();

            let iter = cmd_resp
                .split("&%nm%")
                .filter(|each_message| each_message.len() >= 1);

            for message in iter {
                if let Err(why) = msg.channel_id.say(&ctx.http, message) {
                    println!("Error sending message: {:?}", why);
                }
            }

            println!(
                "Response for command {} found in database: {}",
                command_without_prefix, cmd_resp
            );

            cmd_found = true;
        } else {
            println!(
                "Command {} not found in firebase database",
                command_without_prefix
            );
            cmd_found = false;
        }
    }

    if msg.content.starts_with(prefix!()) && !cmd_found {
        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, "This command doesn't exist, yet! ¯\\_(ツ)_/¯")
        {
            println!("Error sending message: {:?}", why);
        }
    };
}
