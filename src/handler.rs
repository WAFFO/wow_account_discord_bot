use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::model::id::{GuildId, ChannelId};
use serenity::model::user::User;

use crate::account_bot::AccountBot;
use mysql_async::Pool;

pub struct Handler {
    pub(crate) bot: AccountBot,
    pub(crate) db_pool: Pool,
    pub(crate) account_channel: ChannelId,
    pub(crate) leavers_channel: ChannelId,
    pub(crate) whois_channel:   ChannelId,
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_removal(&self, ctx: Context, _: GuildId, user: User) {
        let db_res = self.db_pool.get_conn().await;
        let err_msg = "Error sending leaver message.";
        if let Ok(mut db) = db_res {
            let account = self.bot.get_account_id(&mut db, user.id).await;

            if let Ok(account_id) = account {
                if let Some(id) = account_id {
                    self.leavers_channel.say(ctx, format!(
                        "<@{}> left, their account id is: {}",
                        user.id.0,
                        id,
                    )).await.expect(err_msg);
                } else {
                    self.leavers_channel.say(ctx, format!(
                        "<@{}> left, they did not have an account.",
                        user.id.0,
                    )).await.expect(err_msg);
                }
            } else {
                self.leavers_channel.say(ctx, format!(
                    "<@{}> left, but my connection to the database was interrupted.",
                    user.id.0,
                )).await.expect(err_msg);
            }

            if let Err(_) = db.disconnect().await {
                println!("db disconnect error");
            }
        }
        else {
            self.leavers_channel.say(ctx, format!(
                "<@{}> left, but I cannot connect to the database.",
                user.id.0,
            )).await.expect(err_msg);
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let db_res = self.db_pool.get_conn().await;
        if let Ok(mut db) = db_res {
            if let Err(_) =
            if msg.channel_id == self.account_channel {
                self.bot.manage_account(&mut db, ctx, msg).await
            } else if msg.channel_id == self.whois_channel {
                self.bot.whois(&mut db, ctx, msg).await
            } else {
                Ok(())
            }
            {
                println!("DB DOWN!");
            }

            if let Err(_) = db.disconnect().await {
                println!("db disconnect error");
            }
        }
        else {
            println!("DB DOWN!");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}