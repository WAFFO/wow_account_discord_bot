use std::env;

use serenity::model::id::ChannelId;
use serenity::prelude::*;

use handler::Handler;

mod handler;
mod manage;


#[tokio::main]
async fn main() {

    let discord_token: String = env::var("DISCORD_TOKEN").expect("Missing env variable DISCORD_TOKEN");
    let account_url: String = env::var("SITE_URL").expect("Missing env variable IP_ADDRESS");
    let db_access: String = env::var("DB_URL").expect("Missing env variable DB_URL");

    let account_bot_channel: u64 = env::var("ACCOUNT_BOT_CHANNEL").expect("Missing env variable ACCOUNT_BOT_CHANNEL").parse().unwrap();
    let leavers_channel: u64 = env::var("LEAVERS_CHANNEL").expect("Missing env variable LEAVERS_CHANNEL").parse().unwrap();
    let whois_channel: u64 = env::var("WHOIS_CHANNEL").expect("Missing env variable WHOIS_CHANNEL").parse().unwrap();

    let pool = mysql_async::Pool::new(db_access.as_str());

    let mut client = Client::new(&discord_token)
        .event_handler(Handler{
            db_pool: pool,
            account_channel: ChannelId(account_bot_channel),
            leavers_channel: ChannelId(leavers_channel),
            whois_channel: ChannelId(whois_channel),
            site_url: account_url,
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
