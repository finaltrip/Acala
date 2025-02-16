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
//! HOSTNAME: `ip-172-31-38-126`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/mandala/src/weights/

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
		//  Measured:  `1200`
		//  Estimated: `3660`
		// Minimum execution time: 18_643 nanoseconds.
		Weight::from_parts(19_065_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Dex::TradingPairStatuses` (r:1 w:1)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	fn disable_trading_pair() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1200`
		//  Estimated: `3660`
		// Minimum execution time: 18_275 nanoseconds.
		Weight::from_parts(18_864_000, 3660)
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
		//  Measured:  `1360`
		//  Estimated: `3660`
		// Minimum execution time: 27_957 nanoseconds.
		Weight::from_parts(28_380_000, 3660)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Dex::TradingPairStatuses` (r:1 w:1)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	fn update_provisioning_parameters() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `969`
		//  Estimated: `3660`
		// Minimum execution time: 13_153 nanoseconds.
		Weight::from_parts(13_548_000, 3660)
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
		//  Measured:  `2607`
		//  Estimated: `3660`
		// Minimum execution time: 47_453 nanoseconds.
		Weight::from_parts(48_463_000, 3660)
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
		//  Measured:  `2843`
		//  Estimated: `6234`
		// Minimum execution time: 90_569 nanoseconds.
		Weight::from_parts(91_816_000, 6234)
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
		//  Measured:  `2998`
		//  Estimated: `6272`
		// Minimum execution time: 67_056 nanoseconds.
		Weight::from_parts(68_994_000, 6272)
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
		//  Measured:  `3165`
		//  Estimated: `8856`
		// Minimum execution time: 110_200 nanoseconds.
		Weight::from_parts(113_197_000, 8856)
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
		//  Measured:  `3605`
		//  Estimated: `11478`
		// Minimum execution time: 142_461 nanoseconds.
		Weight::from_parts(144_933_000, 11478)
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
		//  Measured:  `3080`
		//  Estimated: `8856`
		// Minimum execution time: 102_467 nanoseconds.
		Weight::from_parts(105_483_000, 8856)
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
		//  Measured:  `3779`
		//  Estimated: `11478`
		// Minimum execution time: 150_564 nanoseconds.
		Weight::from_parts(152_551_000, 11478)
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
		//  Measured:  `2838 + u * (108 ±0)`
		//  Estimated: `6234 + u * (643 ±19)`
		// Minimum execution time: 89_781 nanoseconds.
		Weight::from_parts(66_696_308, 6234)
			// Standard Error: 74_213
			.saturating_add(Weight::from_parts(12_809_217, 0).saturating_mul(u.into()))
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
		//  Measured:  `2838 + u * (108 ±0)`
		//  Estimated: `6234 + u * (643 ±19)`
		// Minimum execution time: 89_650 nanoseconds.
		Weight::from_parts(67_390_443, 6234)
			// Standard Error: 81_401
			.saturating_add(Weight::from_parts(12_625_344, 0).saturating_mul(u.into()))
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
		//  Measured:  `3170`
		//  Estimated: `6234`
		// Minimum execution time: 98_955 nanoseconds.
		Weight::from_parts(101_480_000, 6234)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: `Dex::TradingPairStatuses` (r:1 w:1)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	fn abort_provisioning() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1338`
		//  Estimated: `3660`
		// Minimum execution time: 22_673 nanoseconds.
		Weight::from_parts(23_292_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
