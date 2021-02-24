use serenity::{async_trait, builder::CreateEmbed, cache, framework::{self, standard::{Args, CommandError, Delimiter, macros::hook}}, http::Http, model::Permissions};
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};
use log::{error, debug};

use std::{env};

mod wowhead;
// mod error;

#[group]
#[commands(item)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$")) // set the bot's prefix to "~"
        .after(after_hook_logger)
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let permissions = Permissions::SEND_MESSAGES | Permissions::ADD_REACTIONS;
    let user = client.cache_and_http.cache.current_user().await;
    match user.invite_url(&client.cache_and_http.http, permissions).await {
        Ok(invite_url) => println!("Invite URL: {}", invite_url),
        Err(e) => println!("Error creating invite URL: {}", e),        
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn item(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);

    let id = args.advance().parse::<u32>()?;

    // msg.content
    let url = format!("https://classic.wowhead.com/item={}&xml", id);

    let xml = reqwest::get(&url)
        .await?
        .text()
        .await?;

    let wowhead_response: wowhead::Wowhead = quick_xml::de::from_str(&xml)?;

    debug!("Wowhead response: {:?}", wowhead_response.item.item_class);

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|embed| {
            embed
                .title(&wowhead_response.item.name)
                .description("This item is awesome.")
        })
    }).await?;

    Ok(())
}


#[hook]
async fn after_hook_logger(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    //  Print out an error if it happened
    if let Err(why) = error {
        error!("Error in command '{}': {:?}", cmd_name, why);

        let mut error_chain = why.source();
        while let Some(error_source) = error_chain {
            error!("Because of: {:?}", error_source);
            error_chain = error_source.source();
        }
    }
}
