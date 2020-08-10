#![feature(try_blocks)]

use super::*;

pub fn run(msg: &serenity::model::channel::Message, ctx: &serenity::client::Context) -> Result<(),std::option::NoneError> {
    let code = crop_letters(&msg.content, 5);

    println!("code : {}", code);

    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let code = v8::String::new(scope, code)?;
    println!("js code: {}", code.to_rust_string_lossy(scope));

    let mut script =  v8::Script::compile(scope, code, None)?;
    let result = script.run(scope)?;
    let result = result.to_string(scope)?;

    println!("Result: {}", result.to_rust_string_lossy(scope));

    if let Err(why) = msg.channel_id.say(
        &ctx.http,
        format!("Result: \n{}", result.to_rust_string_lossy(scope)),
    ) {
        println!("Error sending message: {:?}", why);
    };
}
