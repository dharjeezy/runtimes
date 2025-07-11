// Copyright (C) Parity Technologies and the various Polkadot contributors, see Contributions.md
// for a list of specific contributors.
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

//! Autogenerated weights for `pallet_assets`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 47.2.0
//! DATE: 2025-06-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --runtime=target/production/wbuild/asset-hub-kusama-runtime/asset_hub_kusama_runtime.compact.compressed.wasm
// --header=.github/scripts/cmd/file_header.txt
// --output=./system-parachains/asset-hubs/asset-hub-kusama/src/weights/
// --all
// --quiet

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_assets`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_assets::WeightInfo for WeightInfo<T> {
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::NextAssetId` (r:1 w:0)
	/// Proof: `Assets::NextAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `3675`
		// Minimum execution time: 32_910_000 picoseconds.
		Weight::from_parts(33_491_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::NextAssetId` (r:1 w:0)
	/// Proof: `Assets::NextAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3675`
		// Minimum execution time: 13_980_000 picoseconds.
		Weight::from_parts(14_360_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	fn start_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `3675`
		// Minimum execution time: 14_470_000 picoseconds.
		Weight::from_parts(14_820_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1001 w:1000)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1000 w:1000)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1000]`.
	/// The range of component `c` is `[0, 1000]`.
	/// The range of component `c` is `[0, 1000]`.
	fn destroy_accounts(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `122 + c * (208 ±0)`
		//  Estimated: `3675 + c * (2609 ±0)`
		// Minimum execution time: 19_260_000 picoseconds.
		Weight::from_parts(19_471_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			// Standard Error: 17_973
			.saturating_add(Weight::from_parts(16_574_080, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2609).saturating_mul(c.into()))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Approvals` (r:1001 w:1000)
	/// Proof: `Assets::Approvals` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy_approvals(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `451 + a * (86 ±0)`
		//  Estimated: `3675 + a * (2623 ±0)`
		// Minimum execution time: 19_990_000 picoseconds.
		Weight::from_parts(20_340_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			// Standard Error: 8_102
			.saturating_add(Weight::from_parts(18_095_917, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 2623).saturating_mul(a.into()))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:0)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(140), added: 2615, mode: `MaxEncodedLen`)
	fn finish_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3675`
		// Minimum execution time: 15_640_000 picoseconds.
		Weight::from_parts(16_210_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3675`
		// Minimum execution time: 26_680_000 picoseconds.
		Weight::from_parts(27_230_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `3675`
		// Minimum execution time: 35_800_000 picoseconds.
		Weight::from_parts(36_530_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `6208`
		// Minimum execution time: 50_380_000 picoseconds.
		Weight::from_parts(51_381_000, 0)
			.saturating_add(Weight::from_parts(0, 6208))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `6208`
		// Minimum execution time: 45_360_000 picoseconds.
		Weight::from_parts(45_900_000, 0)
			.saturating_add(Weight::from_parts(0, 6208))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `6208`
		// Minimum execution time: 50_331_000 picoseconds.
		Weight::from_parts(50_950_000, 0)
			.saturating_add(Weight::from_parts(0, 6208))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `3675`
		// Minimum execution time: 19_540_000 picoseconds.
		Weight::from_parts(19_810_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn thaw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `3675`
		// Minimum execution time: 19_300_000 picoseconds.
		Weight::from_parts(20_030_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	fn freeze_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `3675`
		// Minimum execution time: 14_260_000 picoseconds.
		Weight::from_parts(14_620_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	fn thaw_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `3675`
		// Minimum execution time: 14_080_000 picoseconds.
		Weight::from_parts(14_410_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:0)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(140), added: 2615, mode: `MaxEncodedLen`)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3675`
		// Minimum execution time: 15_930_000 picoseconds.
		Weight::from_parts(16_310_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3675`
		// Minimum execution time: 14_100_000 picoseconds.
		Weight::from_parts(14_430_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:1)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(140), added: 2615, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3675`
		// Minimum execution time: 32_310_000 picoseconds.
		Weight::from_parts(33_105_701, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			// Standard Error: 536
			.saturating_add(Weight::from_parts(1_347, 0).saturating_mul(n.into()))
			// Standard Error: 536
			.saturating_add(Weight::from_parts(992, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:1)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(140), added: 2615, mode: `MaxEncodedLen`)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `444`
		//  Estimated: `3675`
		// Minimum execution time: 31_480_000 picoseconds.
		Weight::from_parts(32_200_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:1)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(140), added: 2615, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `119`
		//  Estimated: `3675`
		// Minimum execution time: 14_750_000 picoseconds.
		Weight::from_parts(15_340_727, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			// Standard Error: 323
			.saturating_add(Weight::from_parts(929, 0).saturating_mul(n.into()))
			// Standard Error: 323
			.saturating_add(Weight::from_parts(2_526, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:1)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(140), added: 2615, mode: `MaxEncodedLen`)
	fn force_clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `444`
		//  Estimated: `3675`
		// Minimum execution time: 31_300_000 picoseconds.
		Weight::from_parts(31_560_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	fn force_asset_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3675`
		// Minimum execution time: 13_320_000 picoseconds.
		Weight::from_parts(13_630_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Approvals` (r:1 w:1)
	/// Proof: `Assets::Approvals` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `3675`
		// Minimum execution time: 36_660_000 picoseconds.
		Weight::from_parts(37_090_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Approvals` (r:1 w:1)
	/// Proof: `Assets::Approvals` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `597`
		//  Estimated: `6208`
		// Minimum execution time: 75_550_000 picoseconds.
		Weight::from_parts(76_240_000, 0)
			.saturating_add(Weight::from_parts(0, 6208))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Approvals` (r:1 w:1)
	/// Proof: `Assets::Approvals` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `484`
		//  Estimated: `3675`
		// Minimum execution time: 38_480_000 picoseconds.
		Weight::from_parts(38_981_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Approvals` (r:1 w:1)
	/// Proof: `Assets::Approvals` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	fn force_cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `484`
		//  Estimated: `3675`
		// Minimum execution time: 38_350_000 picoseconds.
		Weight::from_parts(38_791_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	fn set_min_balance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3675`
		// Minimum execution time: 14_890_000 picoseconds.
		Weight::from_parts(15_290_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn touch() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `383`
		//  Estimated: `3675`
		// Minimum execution time: 38_190_000 picoseconds.
		Weight::from_parts(38_490_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	fn touch_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3675`
		// Minimum execution time: 35_800_000 picoseconds.
		Weight::from_parts(36_100_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `509`
		//  Estimated: `3675`
		// Minimum execution time: 36_411_000 picoseconds.
		Weight::from_parts(37_091_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	fn refund_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `439`
		//  Estimated: `3675`
		// Minimum execution time: 34_330_000 picoseconds.
		Weight::from_parts(34_870_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `3675`
		// Minimum execution time: 19_650_000 picoseconds.
		Weight::from_parts(19_930_000, 0)
			.saturating_add(Weight::from_parts(0, 3675))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `6208`
		// Minimum execution time: 60_691_000 picoseconds.
		Weight::from_parts(61_420_000, 0)
			.saturating_add(Weight::from_parts(0, 6208))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
