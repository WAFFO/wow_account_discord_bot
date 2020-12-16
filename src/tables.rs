
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
