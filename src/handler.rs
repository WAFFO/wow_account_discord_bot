use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::model::id::GuildId;
use serenity::model::user::User;

use crate::account_bot::AccountBot;

pub struct Handler {
    pub(crate) bot: AccountBot,
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_removal(&self, ctx: Context, _: GuildId, user: User) {
        let account = self.bot.get_account_id(user.id).await;

        if let Ok(account_id) = account {
            if let Some(id) = account_id {
                self.bot.leavers_channel.say(ctx, format!(
                    "<@{}> left, their account id is: {}",
                    user.id.0,
                    id,
                )).await.expect("Error sending leaver message.");
            } else {
                self.bot.leavers_channel.say(ctx, format!(
                    "<@{}> left, they did not have an account.",
                    user.id.0,
                )).await.expect("Error sending leaver message.");
            }
        }
        else {
            self.bot.leavers_channel.say(ctx, format!(
                "<@{}> left, but I can't connect to the database.",
                user.id.0,
            )).await.expect("Error sending leaver message.");
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(_) =
            if msg.channel_id == self.bot.account_channel {
                self.bot.manage_account(ctx, msg).await
            }
            else if msg.channel_id == self.bot.whois_channel {
                self.bot.whois(ctx, msg).await
            }
            else {
                Ok(())
            }
        {
            println!("DB DOWN!");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}