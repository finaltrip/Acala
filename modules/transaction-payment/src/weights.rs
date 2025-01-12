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

//! Autogenerated weights for module_transaction_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_transaction_payment
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./modules/transaction-payment/src/weights.rs
// --template=./templates/module-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_transaction_payment.
pub trait WeightInfo {
	fn set_alternative_fee_swap_path() -> Weight;
	fn enable_charge_fee_pool() -> Weight;
	fn disable_charge_fee_pool() -> Weight;
	fn on_finalize() -> Weight;
	fn with_fee_path() -> Weight;
	fn with_fee_aggregated_path() -> Weight;
	fn with_fee_currency() -> Weight;
}

/// Weights for module_transaction_payment using the Acala node and recommended hardware.
pub struct AcalaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AcalaWeight<T> {
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: TransactionPayment AlternativeFeeSwapPath (r:0 w:1)
	fn set_alternative_fee_swap_path() -> Weight {
		Weight::from_parts(21_367_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: TransactionPayment PoolSize (r:1 w:1)
	// Storage: TransactionPayment AlternativeFeeSwapPath (r:1 w:0)
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:2)
	// Storage: TransactionPayment TokenExchangeRate (r:0 w:1)
	// Storage: TransactionPayment SwapBalanceThreshold (r:0 w:1)
	fn enable_charge_fee_pool() -> Weight {
		Weight::from_parts(62_403_000, 0)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: TransactionPayment TokenExchangeRate (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:2)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: TransactionPayment SwapBalanceThreshold (r:0 w:1)
	// Storage: TransactionPayment PoolSize (r:0 w:1)
	// Storage: EvmAccounts Accounts (r:0 w:1)
	fn disable_charge_fee_pool() -> Weight {
		Weight::from_parts(66_491_000, 0)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(9 as u64))
	}
	fn with_fee_path() -> Weight {
		Weight::from_parts(156_000_000, 0)
	}
	fn with_fee_aggregated_path() -> Weight {
		Weight::from_parts(156_000_000, 0)
	}
	fn with_fee_currency() -> Weight {
		Weight::from_parts(193_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:1)
	// Storage: System BlockWeight (r:1 w:0)
	fn on_finalize() -> Weight {
		Weight::from_parts(6_779_000, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn set_alternative_fee_swap_path() -> Weight {
		Weight::from_parts(21_367_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	fn enable_charge_fee_pool() -> Weight {
		Weight::from_parts(62_403_000, 0)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	fn disable_charge_fee_pool() -> Weight {
		Weight::from_parts(66_491_000, 0)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(9 as u64))
	}
	fn on_finalize() -> Weight {
		Weight::from_parts(6_779_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn with_fee_path() -> Weight {
		Weight::from_parts(156_000_000, 0)
	}
	fn with_fee_aggregated_path() -> Weight {
		Weight::from_parts(156_000_000, 0)
	}
	fn with_fee_currency() -> Weight {
		Weight::from_parts(193_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
}
