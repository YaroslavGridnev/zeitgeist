//! Autogenerated weights for zrml_swaps
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-04-14, STEPS: `[8, ]`, REPEAT: 5000, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("battery_park"), DB CACHE: 128

// Executed Command:
// ./target/release/zeitgeist
// benchmark
// --chain
// battery_park
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// zrml-swaps
// --extrinsic
// *
// --steps
// 8
// --repeat
// 5000
// --template
// ./templates/weight_template.hbs
// --output
// ./zrml/swaps/src/weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

///  Trait containing the required functions for weight retrival within
/// zrml_swaps (automatically generated)
pub trait WeightInfoZeitgeist {
    fn create_pool(a: u32) -> Weight;
    fn pool_exit(a: u32) -> Weight;
    fn pool_exit_with_exact_asset_amount() -> Weight;
    fn pool_exit_with_exact_pool_amount() -> Weight;
    fn pool_join(a: u32) -> Weight;
    fn pool_join_with_exact_asset_amount() -> Weight;
    fn pool_join_with_exact_pool_amount() -> Weight;
    fn swap_exact_amount_in() -> Weight;
    fn swap_exact_amount_out() -> Weight;
}

/// Weight functions for zrml_swaps (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfoZeitgeist for WeightInfo<T> {
    fn create_pool(a: u32) -> Weight {
        (73_630_000 as Weight)
            // Standard Error: 15_000
            .saturating_add((59_807_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(a as Weight)))
    }
    fn pool_exit(a: u32) -> Weight {
        (67_930_000 as Weight)
            // Standard Error: 14_000
            .saturating_add((57_786_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(a as Weight)))
    }
    fn pool_exit_with_exact_asset_amount() -> Weight {
        (112_953_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn pool_exit_with_exact_pool_amount() -> Weight {
        (111_791_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn pool_join(a: u32) -> Weight {
        (52_148_000 as Weight)
            // Standard Error: 10_000
            .saturating_add((53_031_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(a as Weight)))
    }
    fn pool_join_with_exact_asset_amount() -> Weight {
        (106_341_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn pool_join_with_exact_pool_amount() -> Weight {
        (105_750_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn swap_exact_amount_in() -> Weight {
        (161_275_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn swap_exact_amount_out() -> Weight {
        (162_326_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
}
