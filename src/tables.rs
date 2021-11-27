use mysql_async::prelude::*;
use mysql_async::{FromRowError, Row};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Access {
    discord_id: u64,
    token: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bridge {
    pub discord_id: u64,
    pub account_id: u32,
}

impl FromRow for Bridge {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        let account_id: u32 = row.get("account_id").expect("Got bad account_id type");
        let discord_id: u64 = row.get("discord_id").expect("Got bad discord_id type");
        Ok(Bridge {
            account_id,
            discord_id,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Character {
    pub account: u32,
    pub name: String,
    pub race: u8,
    pub class: u8,
    pub level: u8,
    pub map: u16,
}
