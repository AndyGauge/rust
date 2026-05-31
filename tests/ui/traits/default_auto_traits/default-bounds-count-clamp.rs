// Checks that a `-Zexperimental-default-bounds-count` above the 16-trait pool
// clamps to 16 instead of panicking / indexing out of bounds, and that the
// 16th default trait is active under the clamp. `OptOut16` opts out of only the
// 16th default trait, so it must be rejected (which it can only be if count=200
// was clamped to 16, with no ICE).
//
//@ compile-flags: -Zexperimental-default-bounds -Zexperimental-default-bounds-count=200

#![feature(auto_traits, lang_items, negative_impls, no_core)]
#![allow(incomplete_features, private_bounds)]
#![no_std]
#![no_core]

#[lang = "pointee_sized"]
trait PointeeSized {}
#[lang = "meta_sized"]
trait MetaSized: PointeeSized {}
#[lang = "sized"]
trait Sized: MetaSized {}
#[lang = "copy"]
pub trait Copy {}

#[lang = "default_trait1"]
auto trait Da1 {}
#[lang = "default_trait2"]
auto trait Da2 {}
#[lang = "default_trait3"]
auto trait Da3 {}
#[lang = "default_trait4"]
auto trait Da4 {}
#[lang = "default_trait5"]
auto trait Da5 {}
#[lang = "default_trait6"]
auto trait Da6 {}
#[lang = "default_trait7"]
auto trait Da7 {}
#[lang = "default_trait8"]
auto trait Da8 {}
#[lang = "default_trait9"]
auto trait Da9 {}
#[lang = "default_trait10"]
auto trait Da10 {}
#[lang = "default_trait11"]
auto trait Da11 {}
#[lang = "default_trait12"]
auto trait Da12 {}
#[lang = "default_trait13"]
auto trait Da13 {}
#[lang = "default_trait14"]
auto trait Da14 {}
#[lang = "default_trait15"]
auto trait Da15 {}
#[lang = "default_trait16"]
auto trait Da16 {}

// Auto-implements Da1..=Da15 structurally; opts out of only the 16th.
struct OptOut16;
impl !Da16 for OptOut16 {}

fn needs<T>(_: T) {}

fn main() {
    needs(OptOut16);
    //~^ ERROR the trait bound `OptOut16: Da16` is not satisfied
}
