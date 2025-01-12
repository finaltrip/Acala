// This file is part of Acala.

// Copyright (C) 2020-2025 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_prices
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 35.0.1
//! DATE: 2024-04-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-41-141`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_prices.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_prices::WeightInfo for WeightInfo<T> {
	// Storage: `AcalaOracle::Values` (r:1 w:0)
	// Proof: `AcalaOracle::Values` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::AssetMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::AssetMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Prices::LockedPrice` (r:0 w:1)
	// Proof: `Prices::LockedPrice` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn lock_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1422`
		//  Estimated: `4887`
		// Minimum execution time: 23_614 nanoseconds.
		Weight::from_parts(24_266_000, 4887)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Prices::LockedPrice` (r:1 w:1)
	// Proof: `Prices::LockedPrice` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn unlock_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1123`
		//  Estimated: `3532`
		// Minimum execution time: 17_142 nanoseconds.
		Weight::from_parts(17_897_000, 3532)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
