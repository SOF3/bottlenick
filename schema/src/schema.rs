table! {
    exemptions (guild_id, role) {
        guild_id -> Int8,
        role -> Int8,
        warmup -> Interval,
        cooldown -> Interval,
    }
}

table! {
    guild_members (id, guild_id) {
        id -> Int8,
        guild_id -> Int8,
        warmup_time -> Nullable<Timestamp>,
        cooldown_time -> Nullable<Timestamp>,
    }
}

table! {
    guilds (id) {
        id -> Int8,
        throttle_warmup -> Interval,
        throttle_cooldown -> Interval,
    }
}

joinable!(exemptions -> guilds (guild_id));
joinable!(guild_members -> guilds (guild_id));

allow_tables_to_appear_in_same_query!(exemptions, guild_members, guilds,);
