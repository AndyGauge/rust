// Checks that `-Zexperimental-default-bounds-count=N` controls how many of the
// default auto traits (`default_trait1..16`) actually become implicit bounds:
// only the first `N` are active. `OptOut7` opts out of just the 7th, so it is
// accepted when N=4 (trait 7 inactive) and rejected when N=7 (trait 7 active).
//
//@ revisions: count4 count7
//@ [count4] compile-flags: -Zexperimental-default-bounds -Zexperimental-default-bounds-count=4
//@ [count7] compile-flags: -Zexperimental-default-bounds -Zexperimental-default-bounds-count=7
//@ [count4] check-pass

#![feature(auto_traits, lang_items, negative_impls, no_core)]
// The no_core scaffold's `pub trait Copy` picks up `pub(crate)` default-trait
// bounds, which is irrelevant to what this test checks.
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

// Auto-implements Da1..=Da6 structurally; opts out of only the 7th.
struct OptOut7;
impl !Da7 for OptOut7 {}

fn needs<T>(_: T) {}

fn main() {
    needs(OptOut7);
    //[count7]~^ ERROR the trait bound `OptOut7: Da7` is not satisfied
}
