
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

pub trait WeightInfo {
  fn create_ownership() -> Weight;
  fn remove_ownership() -> Weight;
  fn get_owned_assets() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
  fn create_ownership() -> Weight {
    // Proof Size summary in bytes:
    //  Measured:  `0`
    //  Estimated: `0`
    // Minimum execution time: 8_000_000 picoseconds.
    Weight::from_parts(10_000_000, 0)
      .saturating_add(T::DbWeight::get().writes(1_u64))
  }

    fn remove_ownership() -> Weight {
    // Proof Size summary in bytes:
    //  Measured:  `32`
    //  Estimated: `1489`
    // Minimum execution time: 6_000_000 picoseconds.
    Weight::from_parts(10_000_000, 1489)
      .saturating_add(T::DbWeight::get().reads(1_u64))
      .saturating_add(T::DbWeight::get().writes(1_u64))
  }

  fn get_owned_assets() -> Weight {
    // Proof Size summary in bytes:
    //  Measured:  `0`
    //  Estimated: `0`
    // Minimum execution time: 8_000_000 picoseconds.
    Weight::from_parts(10_000_000, 0)
      .saturating_add(T::DbWeight::get().writes(1_u64))
  }
}

// For backwards compatibility and tests
impl WeightInfo for () {
  fn create_ownership() -> Weight {
    // Proof Size summary in bytes:
    //  Measured:  `0`
    //  Estimated: `0`
    // Minimum execution time: 8_000_000 picoseconds.
    Weight::from_parts(10_000_000, 0)
      .saturating_add(RocksDbWeight::get().writes(1_u64))
  }

    fn remove_ownership() -> Weight {
    // Proof Size summary in bytes:
    //  Measured:  `32`
    //  Estimated: `1489`
    // Minimum execution time: 6_000_000 picoseconds.
    Weight::from_parts(10_000_000, 1489)
      .saturating_add(RocksDbWeight::get().reads(1_u64))
      .saturating_add(RocksDbWeight::get().writes(1_u64))
  }

  fn get_owned_assets() -> Weight {
    // Proof Size summary in bytes:
    //  Measured:  `0`
    //  Estimated: `0`
    // Minimum execution time: 8_000_000 picoseconds.
    Weight::from_parts(10_000_000, 0)
      .saturating_add(RocksDbWeight::get().writes(1_u64))
  }
}