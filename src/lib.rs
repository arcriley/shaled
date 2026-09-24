/*  src/lib.rs  Library entry point and primary interfaces for shaled.
 *
 *  Copyright 2026 Emerge Cooperative
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published
 *  by the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *                                                                          */

//! `shaled` is a layer-streaming tensor execution engine designed to execute
//! massive (>100GB) 4-bit model files slice-by-slice directly from disk into RAM.

pub mod error;

pub use error::ShaledError;

/// Placeholder 4-bit unsigned integer type alias targeting `nibbles` 1.0 specifications.
pub type U4 = u8;
