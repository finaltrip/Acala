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

//! Autogenerated weights for module_aggregated_dex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/acala
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_aggregated_dex
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./modules/aggregated-dex/src/weights.rs
// --template=./templates/module-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_aggregated_dex.
pub trait WeightInfo {
	fn swap_with_exact_supply(u: u32, ) -> Weight;
	fn swap_with_exact_target(u: u32, ) -> Weight;
	fn update_aggregated_swap_paths(u: u32, ) -> Weight;
}

/// Weights for module_aggregated_dex using the Acala node and recommended hardware.
pub struct AcalaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AcalaWeight<T> {
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		Weight::from_parts(70_917_000, 0)
			// Standard Error: 1_041_000
			.saturating_add(Weight::from_parts(8_550_000, 0).saturating_mul(u as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(u as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(u as u64)))
	}
	fn swap_with_exact_target(u: u32, ) -> Weight {
		Weight::from_parts(58_717_000, 0)
			// Standard Error: 147_000
			.saturating_add(Weight::from_parts(15_650_000, 0).saturating_mul(u as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(u as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(u as u64)))
	}
	fn update_aggregated_swap_paths(n: u32, ) -> Weight {
		Weight::from_parts(4_558_000, 0)
			// Standard Error: 25_000
			.saturating_add(Weight::from_parts(1_533_000, 0).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		Weight::from_parts(70_917_000, 0)
			// Standard Error: 1_041_000
			.saturating_add(Weight::from_parts(8_550_000, 0).saturating_mul(u as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(u as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(u as u64)))
	}
	fn swap_with_exact_target(u: u32, ) -> Weight {
		Weight::from_parts(58_717_000, 0)
			// Standard Error: 147_000
			.saturating_add(Weight::from_parts(15_650_000, 0).saturating_mul(u as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(u as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(u as u64)))
	}
	fn update_aggregated_swap_paths(n: u32, ) -> Weight {
		Weight::from_parts(4_558_000, 0)
			// Standard Error: 25_000
			.saturating_add(Weight::from_parts(1_533_000, 0).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}
}
