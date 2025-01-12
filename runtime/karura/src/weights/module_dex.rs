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

//! Autogenerated weights for module_dex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 35.0.1
//! DATE: 2024-04-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-40-129`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_dex.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_dex::WeightInfo for WeightInfo<T> {
	// Storage: `Dex::TradingPairStatuses` (r:1 w:1)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	fn enable_trading_pair() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1074`
		//  Estimated: `3660`
		// Minimum execution time: 15_593 nanoseconds.
		Weight::from_parts(15_943_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Dex::TradingPairStatuses` (r:1 w:1)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	fn disable_trading_pair() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `3660`
		// Minimum execution time: 16_448 nanoseconds.
		Weight::from_parts(16_986_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Dex::TradingPairStatuses` (r:1 w:1)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Dex::ProvisioningPool` (r:1 w:0)
	// Proof: `Dex::ProvisioningPool` (`max_values`: None, `max_size`: Some(166), added: 2641, mode: `MaxEncodedLen`)
	fn list_provisioning() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1116`
		//  Estimated: `3660`
		// Minimum execution time: 22_370 nanoseconds.
		Weight::from_parts(23_019_000, 3660)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Dex::TradingPairStatuses` (r:1 w:1)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	fn update_provisioning_parameters() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `915`
		//  Estimated: `3660`
		// Minimum execution time: 10_584 nanoseconds.
		Weight::from_parts(11_032_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Dex::TradingPairStatuses` (r:1 w:1)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:1 w:1)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Dex::LiquidityPool` (r:1 w:1)
	// Proof: `Dex::LiquidityPool` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	// Storage: `Dex::InitialShareExchangeRates` (r:0 w:1)
	// Proof: `Dex::InitialShareExchangeRates` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	fn end_provisioning() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1708`
		//  Estimated: `3660`
		// Minimum execution time: 41_825 nanoseconds.
		Weight::from_parts(43_270_000, 3660)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: `Dex::TradingPairStatuses` (r:1 w:1)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	// Storage: `Dex::ProvisioningPool` (r:1 w:1)
	// Proof: `Dex::ProvisioningPool` (`max_values`: None, `max_size`: Some(166), added: 2641, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	fn add_provision() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1867`
		//  Estimated: `6234`
		// Minimum execution time: 84_089 nanoseconds.
		Weight::from_parts(85_638_000, 6234)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: `Dex::TradingPairStatuses` (r:1 w:0)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	// Storage: `Dex::ProvisioningPool` (r:2 w:1)
	// Proof: `Dex::ProvisioningPool` (`max_values`: None, `max_size`: Some(166), added: 2641, mode: `MaxEncodedLen`)
	// Storage: `Dex::InitialShareExchangeRates` (r:1 w:1)
	// Proof: `Dex::InitialShareExchangeRates` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn claim_dex_share() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2062`
		//  Estimated: `6272`
		// Minimum execution time: 61_816 nanoseconds.
		Weight::from_parts(63_135_000, 6272)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: `Dex::TradingPairStatuses` (r:1 w:0)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	// Storage: `Dex::LiquidityPool` (r:1 w:1)
	// Proof: `Dex::LiquidityPool` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:3 w:3)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2200`
		//  Estimated: `8856`
		// Minimum execution time: 101_565 nanoseconds.
		Weight::from_parts(103_440_000, 8856)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: `Dex::TradingPairStatuses` (r:1 w:0)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	// Storage: `Dex::LiquidityPool` (r:1 w:1)
	// Proof: `Dex::LiquidityPool` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:4 w:4)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `Rewards::PoolInfos` (r:1 w:1)
	// Proof: `Rewards::PoolInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Rewards::SharesAndWithdrawnRewards` (r:1 w:1)
	// Proof: `Rewards::SharesAndWithdrawnRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn add_liquidity_and_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2607`
		//  Estimated: `11478`
		// Minimum execution time: 134_425 nanoseconds.
		Weight::from_parts(137_904_000, 11478)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: `Dex::LiquidityPool` (r:1 w:1)
	// Proof: `Dex::LiquidityPool` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:3 w:3)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2134`
		//  Estimated: `8856`
		// Minimum execution time: 97_154 nanoseconds.
		Weight::from_parts(98_814_000, 8856)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: `Dex::LiquidityPool` (r:1 w:1)
	// Proof: `Dex::LiquidityPool` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Rewards::SharesAndWithdrawnRewards` (r:1 w:1)
	// Proof: `Rewards::SharesAndWithdrawnRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:4 w:4)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Rewards::PoolInfos` (r:1 w:1)
	// Proof: `Rewards::PoolInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	fn remove_liquidity_by_unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2547`
		//  Estimated: `11478`
		// Minimum execution time: 143_142 nanoseconds.
		Weight::from_parts(146_721_000, 11478)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: `Dex::TradingPairStatuses` (r:3 w:0)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	// Storage: `Dex::LiquidityPool` (r:3 w:3)
	// Proof: `Dex::LiquidityPool` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	/// The range of component `u` is `[2, 4]`.
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1810 + u * (112 ±0)`
		//  Estimated: `6234 + u * (643 ±18)`
		// Minimum execution time: 84_306 nanoseconds.
		Weight::from_parts(66_156_863, 6234)
			// Standard Error: 56_488
			.saturating_add(Weight::from_parts(10_354_275, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 643).saturating_mul(u.into()))
	}
	// Storage: `Dex::TradingPairStatuses` (r:3 w:0)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	// Storage: `Dex::LiquidityPool` (r:3 w:3)
	// Proof: `Dex::LiquidityPool` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	/// The range of component `u` is `[2, 4]`.
	fn swap_with_exact_target(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1810 + u * (112 ±0)`
		//  Estimated: `6234 + u * (643 ±18)`
		// Minimum execution time: 84_182 nanoseconds.
		Weight::from_parts(64_800_898, 6234)
			// Standard Error: 51_699
			.saturating_add(Weight::from_parts(10_875_444, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 643).saturating_mul(u.into()))
	}
	// Storage: `Dex::TradingPairStatuses` (r:1 w:0)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	// Storage: `Dex::InitialShareExchangeRates` (r:1 w:0)
	// Proof: `Dex::InitialShareExchangeRates` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	// Storage: `Dex::ProvisioningPool` (r:1 w:1)
	// Proof: `Dex::ProvisioningPool` (`max_values`: None, `max_size`: Some(166), added: 2641, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	fn refund_provision() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2201`
		//  Estimated: `6234`
		// Minimum execution time: 93_287 nanoseconds.
		Weight::from_parts(95_635_000, 6234)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: `Dex::TradingPairStatuses` (r:1 w:1)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	fn abort_provisioning() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1248`
		//  Estimated: `3660`
		// Minimum execution time: 20_424 nanoseconds.
		Weight::from_parts(21_045_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
