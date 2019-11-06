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

use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};

use derive_more::From;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorVariant,
    context: Vec<Cow<'static, str>>,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for ctx in self.context.iter().rev() {
            write!(f, "{}: ", ctx)?;
        }
        write!(f, "{}", self.kind.as_error())?;
        Ok(())
    }
}

impl std::error::Error for Error {}

pub fn ctx<T: Into<Cow<'static, str>>>(c: T) -> impl FnOnce(Error) -> Error {
    move |mut err| {
        err.context.push(c.into());
        err
    }
}

impl<E> From<E> for Error
where
    ErrorVariant: From<E>,
{
    fn from(inner: E) -> Error {
        Error {
            kind: ErrorVariant::from(inner),
            context: vec![],
        }
    }
}

#[derive(Debug, From)]
pub enum ErrorVariant {
    Io(std::io::Error),
    Config(config::ConfigError),
    Serenity(serenity::Error),
    R2d2(r2d2::Error),
}

impl ErrorVariant {
    #[inline]
    fn as_error(&self) -> &dyn std::error::Error {
        match self {
            Self::Io(e) => e,
            Self::Config(e) => e,
            Self::Serenity(e) => e,
            Self::R2d2(e) => e,
        }
    }
}

static_assertions::assert_impl_all!(Error: Send, Sync);
