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

use std::time::SystemTime;

use std::fmt::Debug;

use chrono::Duration;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

use crate::Result;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Db {
    pool: Pool,
}

impl Db {
    pub fn new(url: &str) -> Result<Self> {
        Ok(Self {
            pool: Pool::new(ConnectionManager::new(url))?,
        })
    }

    pub fn check_member_times(
        &self,
        guild: u64,
        user: u64,
    ) -> Result<(Option<SystemTime>, Option<SystemTime>)> {
        unimplemented!("check_member_times, {}, {}", guild, user)
    }

    pub fn get_guild_role_intervals<I>(&self, guild: u64, roles: I) -> Result<(Duration, Duration)>
    where
        I: Iterator<Item = u64> + Debug,
    {
        unimplemented!("get_guild_role_intervals, {}, {:?}", guild, roles)
    }

    pub fn set_member_times(
        &self,
        guild: u64,
        user: u64,
        warmup: Duration,
        cooldown: Duration,
    ) -> Result {
        unimplemented!(
            "set_member_times, {}, {}, {}, {}",
            guild,
            user,
            warmup,
            cooldown
        )
    }
}
