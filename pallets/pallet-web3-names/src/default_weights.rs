// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2024 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Autogenerated weights for pallet_web3_names
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-18
//! STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `rust-2`, CPU: `12th Gen Intel(R) Core(TM) i9-12900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --template=.maintain/weight-template.hbs
// --header=HEADER-GPL
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --steps=50
// --repeat=20
// --chain=dev
// --pallet=pallet-web3-names
// --extrinsic=*
// --output=./pallets/pallet-web3-names/src/default_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_web3_names.
pub trait WeightInfo {
	fn claim(n: u32, ) -> Weight;
	fn release_by_owner() -> Weight;
	fn reclaim_deposit(n: u32, ) -> Weight;
	fn ban(n: u32, ) -> Weight;
	fn unban(n: u32, ) -> Weight;
	fn change_deposit_owner() -> Weight;
	fn update_deposit() -> Weight;
}

/// Weights for pallet_web3_names using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Web3Names Names (r:1 w:1)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: Web3Names Banned (r:1 w:0)
	/// Proof: Web3Names Banned (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// The range of component `n` is `[3, 32]`.
	fn claim(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `10299`
		// Minimum execution time: 19_822 nanoseconds.
		Weight::from_parts(21_345_315, 10299)
			// Standard Error: 17_787
			.saturating_add(Weight::from_parts(23_241, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Web3Names Names (r:1 w:1)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn release_by_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `476`
		//  Estimated: `7775`
		// Minimum execution time: 18_311 nanoseconds.
		Weight::from_parts(19_202_000, 7775)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Web3Names Names (r:0 w:1)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// The range of component `n` is `[3, 32]`.
	fn reclaim_deposit(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `342 + n * (1 ±0)`
		//  Estimated: `5219`
		// Minimum execution time: 16_811 nanoseconds.
		Weight::from_parts(18_600_543, 5219)
			// Standard Error: 15_740
			.saturating_add(Weight::from_parts(12_826, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Web3Names Banned (r:1 w:1)
	/// Proof: Web3Names Banned (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Web3Names Names (r:0 w:1)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// The range of component `n` is `[3, 32]`.
	fn ban(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `342 + n * (1 ±0)`
		//  Estimated: `7743`
		// Minimum execution time: 18_069 nanoseconds.
		Weight::from_parts(19_324_271, 7743)
			// Standard Error: 12_698
			.saturating_add(Weight::from_parts(62_295, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Web3Names Banned (r:1 w:1)
	/// Proof: Web3Names Banned (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `n` is `[3, 32]`.
	fn unban(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `47 + n * (1 ±0)`
		//  Estimated: `2524`
		// Minimum execution time: 7_796 nanoseconds.
		Weight::from_parts(8_240_564, 2524)
			// Standard Error: 5_757
			.saturating_add(Weight::from_parts(47_283, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Web3Names Names (r:1 w:0)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn change_deposit_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `889`
		//  Estimated: `10382`
		// Minimum execution time: 25_771 nanoseconds.
		Weight::from_parts(26_681_000, 10382)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn update_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `374`
		//  Estimated: `5219`
		// Minimum execution time: 22_468 nanoseconds.
		Weight::from_parts(23_034_000, 5219)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Web3Names Names (r:1 w:1)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: Web3Names Banned (r:1 w:0)
	/// Proof: Web3Names Banned (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// The range of component `n` is `[3, 32]`.
	fn claim(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `10299`
		// Minimum execution time: 19_822 nanoseconds.
		Weight::from_parts(21_345_315, 10299)
			// Standard Error: 17_787
			.saturating_add(Weight::from_parts(23_241, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Web3Names Names (r:1 w:1)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn release_by_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `476`
		//  Estimated: `7775`
		// Minimum execution time: 18_311 nanoseconds.
		Weight::from_parts(19_202_000, 7775)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Web3Names Names (r:0 w:1)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// The range of component `n` is `[3, 32]`.
	fn reclaim_deposit(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `342 + n * (1 ±0)`
		//  Estimated: `5219`
		// Minimum execution time: 16_811 nanoseconds.
		Weight::from_parts(18_600_543, 5219)
			// Standard Error: 15_740
			.saturating_add(Weight::from_parts(12_826, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Web3Names Banned (r:1 w:1)
	/// Proof: Web3Names Banned (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	/// Storage: Web3Names Names (r:0 w:1)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// The range of component `n` is `[3, 32]`.
	fn ban(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `342 + n * (1 ±0)`
		//  Estimated: `7743`
		// Minimum execution time: 18_069 nanoseconds.
		Weight::from_parts(19_324_271, 7743)
			// Standard Error: 12_698
			.saturating_add(Weight::from_parts(62_295, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: Web3Names Banned (r:1 w:1)
	/// Proof: Web3Names Banned (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `n` is `[3, 32]`.
	fn unban(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `47 + n * (1 ±0)`
		//  Estimated: `2524`
		// Minimum execution time: 7_796 nanoseconds.
		Weight::from_parts(8_240_564, 2524)
			// Standard Error: 5_757
			.saturating_add(Weight::from_parts(47_283, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Web3Names Names (r:1 w:0)
	/// Proof: Web3Names Names (max_values: None, max_size: Some(81), added: 2556, mode: MaxEncodedLen)
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn change_deposit_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `889`
		//  Estimated: `10382`
		// Minimum execution time: 25_771 nanoseconds.
		Weight::from_parts(26_681_000, 10382)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Web3Names Owner (r:1 w:1)
	/// Proof: Web3Names Owner (max_values: None, max_size: Some(137), added: 2612, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn update_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `374`
		//  Estimated: `5219`
		// Minimum execution time: 22_468 nanoseconds.
		Weight::from_parts(23_034_000, 5219)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}