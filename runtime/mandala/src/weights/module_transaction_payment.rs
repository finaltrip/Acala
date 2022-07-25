// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
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
//! DATE: 2022-05-18, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_transaction_payment
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/mandala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_transaction_payment.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_transaction_payment::WeightInfo for WeightInfo<T> {
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: TransactionPayment AlternativeFeeSwapPath (r:0 w:1)
	fn set_alternative_fee_swap_path() -> Weight {
		(24_608_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: TransactionPayment GlobalFeeSwapPath (r:1 w:1)
	// Storage: TransactionPayment PoolSize (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:2)
	// Storage: TransactionPayment TokenExchangeRate (r:0 w:1)
	// Storage: TransactionPayment SwapBalanceThreshold (r:0 w:1)
	fn enable_charge_fee_pool() -> Weight {
		(70_470_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: TransactionPayment TokenExchangeRate (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:2)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: TransactionPayment SwapBalanceThreshold (r:0 w:1)
	// Storage: TransactionPayment GlobalFeeSwapPath (r:0 w:1)
	// Storage: TransactionPayment PoolSize (r:0 w:1)
	fn disable_charge_fee_pool() -> Weight {
		(69_602_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: TransactionPause PausedTransactions (r:1 w:0)
	fn with_fee_path() -> Weight {
		(4_339_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn with_fee_aggregated_path() -> Weight {
		(4_339_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: TransactionPause PausedTransactions (r:1 w:0)
	fn with_fee_currency() -> Weight {
		(4_121_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: TransactionPause PausedTransactions (r:1 w:0)
	fn with_fee_paid_by() -> Weight {
		(4_118_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:1)
	// Storage: System BlockWeight (r:1 w:0)
	fn on_finalize() -> Weight {
		(6_787_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
