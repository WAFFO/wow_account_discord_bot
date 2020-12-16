use mysql_async::prelude::*;
use random_string::{Charset, RandomString};
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use mysql_async::Conn;

#[allow(dead_code)]
pub async fn get_token(db: &mut Conn, user_id: &UserId) -> Result<String, mysql_async::Error> {
    // clear old tokens
    let _ : Vec<bool> = db.query("call expire_access()").await?;
    // check if token exists
    let result: Option<String> = db.query_first(
        format!("select token from access where discord_id={}", user_id.0)
    ).await?;
    // if exists
    if let Some(token) = result {
        Ok(token)
    }
    else {
        // get new token
        let token = generate_token();
        // insert into database
        let params = params! {
            "discord_id" => user_id.0,
            "token" => token.clone(),
        };
        let _ : Vec<bool> = db.exec(
            "insert into access (discord_id, token) values (:discord_id, :token)",
            params
        ).await?;
        Ok(token)
    }
}

pub async fn manage_account(db: &mut Conn, ctx: Context, msg: Message, site_url: &String) -> Result<(), mysql_async::Error> {
    let user_id = msg.author.id;
    let token = get_token(db, &user_id).await?;
    if let Err(why) = msg.author.direct_message(&ctx, |m| {
        m.content(&format!(
            r"Hello, here is your personal link to create/manage your account:

http://{}/?t={}

This link will only work for ten minutes. Do *not* share this link.",
            site_url,
            token,
        ))
    }).await {
        println!("Error sending message: {:?}", why);
    }
    if let Err(why) = msg.delete(&ctx).await {
        println!("Error deleting message: {:?}", why);
    }
    Ok(())
}

pub async fn whois(db: &mut Conn, ctx: Context, msg: Message) -> Result<(), mysql_async::Error> {
    let mut result = String::from("");
    if msg.content.starts_with(r"<@") && msg.content.ends_with(r">") {
        // lookup by discord user id
        let cap = String::from(msg.content
            .trim_start_matches("<@")
            .trim_start_matches("!")
            .trim_end_matches(">"));

        if let Ok(_) = cap.parse::<u64>() {
            if let Some(account) = get_account_id_str(db, cap.as_str()).await? {
                result = format!("<@{}> account id is: {}", cap, account);
            }
            else {
                result = format!("<@{}> does not have an account.", cap);
            }
        }
        else {
            println!("Could not parse: {}", cap);
        }
    }
    else {
        // lookup by character name
        if let Some(id) = get_user_from_character(db, &msg.content).await? {
            result = format!("{} belongs to <@{}>", msg.content, id);
        }
        else {
            result = format!("No character called \"{}\"", msg.content);
        }
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, result).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

pub async fn get_account_id(db: &mut Conn, user: UserId) -> Result<Option<String>, mysql_async::Error> {
    let result: Option<String> = db.query_first(
        format!("select account_id from bridge where discord_id={}", user.0)
    ).await?;

    Ok(result)
}

pub async fn get_account_id_str(db: &mut Conn, user: &str) -> Result<Option<String>, mysql_async::Error> {
    let result: Option<String> = db.query_first(
        format!("select account_id from bridge where discord_id={}", user)
    ).await?;

    Ok(result)
}

pub async fn get_user_from_character(db: &mut Conn, character_name: &str) -> Result<Option<u64>, mysql_async::Error> {
    let result: Option<u64> = db.query_first(
        format!(
            "select discord_id from bridge where account_id = (\
            select account from acore_characters.characters where LOWER(name)='{}')",
            character_name.to_lowercase(),
        )
    ).await?;

    Ok(result)
}

fn generate_token() -> String {
    let charset = RandomString::get_charset(Charset::Letters);

    match RandomString::generate(8, charset) {
        Ok(string) => string,
        Err(_) => String::from("aaaaaaaaaaa"),
    }
}
