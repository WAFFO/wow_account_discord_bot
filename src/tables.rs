
#[derive(Debug, PartialEq, Eq, Clone)]
struct Access {
    discord_id: String,
    token: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Bridge {
    discord_id: String,
    account_id: u32,
}