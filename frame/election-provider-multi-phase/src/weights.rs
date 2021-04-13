// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_election_provider_multi_phase
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-03-19, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_election_provider_multi_phase
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/election-provider-multi-phase/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_election_provider_multi_phase.
pub trait WeightInfo {
	fn on_initialize_nothing() -> Weight;
	fn on_initialize_open_signed() -> Weight;
	fn on_initialize_open_unsigned_with_snapshot() -> Weight;
	fn on_initialize_open_unsigned_without_snapshot() -> Weight;
	fn elect_queued() -> Weight;
	fn submit_unsigned(v: u32, t: u32, a: u32, d: u32, ) -> Weight;
	fn feasibility_check(v: u32, t: u32, a: u32, d: u32, ) -> Weight;
}

/// Weights for pallet_election_provider_multi_phase using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn on_initialize_nothing() -> Weight {
		(22_730_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
	}
	fn on_initialize_open_signed() -> Weight {
		(112_051_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn on_initialize_open_unsigned_with_snapshot() -> Weight {
		(112_165_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn on_initialize_open_unsigned_without_snapshot() -> Weight {
		(21_039_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn elect_queued() -> Weight {
		(7_362_949_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn submit_unsigned(v: u32, _t: u32, a: u32, d: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 21_000
			.saturating_add((3_933_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 21_000
			.saturating_add((13_520_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 107_000
			.saturating_add((2_880_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn feasibility_check(v: u32, t: u32, a: u32, d: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 10_000
			.saturating_add((4_069_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 36_000
			.saturating_add((503_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 10_000
			.saturating_add((10_000_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 54_000
			.saturating_add((3_734_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn on_initialize_nothing() -> Weight {
		(22_730_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
	}
	fn on_initialize_open_signed() -> Weight {
		(112_051_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn on_initialize_open_unsigned_with_snapshot() -> Weight {
		(112_165_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn on_initialize_open_unsigned_without_snapshot() -> Weight {
		(21_039_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn elect_queued() -> Weight {
		(7_362_949_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn submit_unsigned(v: u32, _t: u32, a: u32, d: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 21_000
			.saturating_add((3_933_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 21_000
			.saturating_add((13_520_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 107_000
			.saturating_add((2_880_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn feasibility_check(v: u32, t: u32, a: u32, d: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 10_000
			.saturating_add((4_069_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 36_000
			.saturating_add((503_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 10_000
			.saturating_add((10_000_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 54_000
			.saturating_add((3_734_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
	}
}
