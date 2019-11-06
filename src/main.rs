// bottlenick
// Copyright (C) SOFe
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affer General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use serenity::client::Client;
use serenity::model::prelude::{Guild, GuildId, Member, RoleId, User};
use serenity::prelude::*;

dirmod::all!(default priv use; except schema);

struct Handler;

impl EventHandler for Handler {
    fn guild_member_update(&self, ctx: Context, old: Option<Member>, new: Member) {
        if let Some(old) = old {
            if old.nick != new.nick {
                match nick_changed(&ctx, new.guild_id, &new.user.read(), &new.roles) {
                    Ok(_) => (),
                    Err(err) => log::error!("Error handling nick change: {}", err),
                }
            }
        }
    }
}

fn nick_changed(ctx: &Context, guild: GuildId, user: &User, roles: &[RoleId]) -> Result {
    let data = ctx.data.read();
    let db = data.get::<data::Db>().expect("data::Db not initialized");

    let (warmup_time, cooldown_time) = db.check_member_times(guild.0, user.id.0)?;
    if warmup_time.is_some() {
        return Ok(());
    }

    if cooldown_time.is_some() {
        log::info!(
            "User#{} {}#{} changed nick during cooldown",
            user.id,
            user.name,
            user.discriminator
        );
        return Ok(());
    }

    let (warmup_interval, cooldown_interval) =
        db.get_guild_role_intervals(guild.0, roles.iter().map(|role| role.0))?;
    db.set_member_times(guild.0, user.id.0, warmup_interval, cooldown_interval)?;

    let guild = Guild::get(ctx, guild)?;
    user.direct_message(ctx, move |m| {
        m.content(&format!(
            "You have just changed your nick in {}.
You may change your nick again with {}.
After that, your nick will be frozen for {}.",
            &guild.name, &warmup_interval, &cooldown_interval,
        ))
    })?;

    Ok(())
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("Fatal error: {}", err);
        std::process::exit(1);
    }
}

fn try_main() -> Result {
    let secrets = Secrets::try_new().map_err(ctx("Reading configuration"))?;

    let db = Db::new(&secrets.database().url())?;

    let mut client = Client::new(&secrets.discord().token(), Handler)?;

    {
        let mut data = client.data.write();
        data.insert::<data::Db>(db);
    }

    log::info!("Invite link: https://discordapp.com/oauth2/authorize?client_id={}&scope=bot&permissions={}", secrets.discord().client_id(), secrets.discord().scope());

    client.start()?;
    Ok(())
}

mod data {
    use serenity::prelude::*;

    pub struct Db;

    impl TypeMapKey for Db {
        type Value = crate::Db;
    }
}
