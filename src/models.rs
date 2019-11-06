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

use diesel::sql_types as types;
use diesel::*;

use schema::*;

#[derive(Queryable, Identifiable, Debug)]
pub struct Guild {
    pub id: u64,
    /// period from first nick change to nick role removal
    pub throttle_warmup: types::Interval,
    /// period from first nick change to nick role addition
    pub throttle_cooldown: types::Interval,
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[primary_key(guild_id, role)]
#[belongs_to(Guild)]
pub struct Exemption {
    pub guild_id: u64,
    pub role: u64,
    pub warmup: types::Interval,
    pub cooldown: types::Interval,
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[primary_key(guild_id, id)]
#[belongs_to(Guild)]
pub struct GuildMember {
    pub guild_id: u64,
    pub id: u64,
    pub warmup_time: Option<types::Timestamp>,
    pub cooldown_time: Option<types::Timestamp>,
}
