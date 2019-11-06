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

//! This crate defines the database schema for bottlenick.
//!
//! This is not merged into the main crate because
//! `diesel` print-schema generates output that requires `#[macro_use]` to be enabled,
//! but that would corrupt mix up with the symbols in the whole crate,
//! so it is separated into an independent crate.

#[macro_use]
extern crate diesel;

include!("schema.rs");
