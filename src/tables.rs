#[derive(Debug, PartialEq, Eq, Clone)]
struct Access {
    discord_id: u64,
    token: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Bridge {
    discord_id: u64,
    account_id: u32,
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
