use serenity::{async_trait, builder::CreateEmbed, cache, framework, http::Http, model::Permissions};
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

use std::env;

mod wowhead;

#[group]
#[commands(item)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$")) // set the bot's prefix to "~"
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
    // msg.reply(ctx, "Pong!").await?;

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|embed| {
            embed
                .title("Wowhead item lookup")
                .description("Vexing Cane")
        })
    }).await?;


    let id = 18082;

    // msg.content
    let url = format!("https://classic.wowhead.com/item={}", id);



    // let xml = match reqwest::get(&url).await {
    //     Ok(response) => {
    //         response.
    //     }
    //     Err(_) => {}
    // }

    let xml = reqwest::get(&url)
        .await?
        .text()
        .await?;

    let wowhead_response: wowhead::Wowhead = quick_xml::de::from_str(&xml)?;


    msg.channel_id.send_message(ctx, |m| {
        m.embed(|embed| {
            embed
                .title("Wowhead item lookup")
                .description(&wowhead_response.item.name)
        })
    }).await?;


    Ok(())
}

