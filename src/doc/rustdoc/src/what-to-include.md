# What to include (and exclude)

It is easy to say everything must be documented in a project and often times
that is correct, but how can we get there, and are there things that don't
belong?

At the top of the `src/lib.rs` or `main.rs` file in your binary project, include
the following attribute:

```rust
#![warn(missing_docs)]
```

Now run `cargo doc` and examine the output.  Here's a sample:

```text
 Documenting docdemo v0.1.0 (/Users/username/docdemo)
warning: missing documentation for the crate
 --> src/main.rs:1:1
  |
1 | / #![warn(missing_docs)]
2 | |
3 | | fn main() {
4 | |     println!("Hello, world!");
5 | | }
  | |_^
  |
note: the lint level is defined here
 --> src/main.rs:1:9
  |
1 | #![warn(missing_docs)]
  |         ^^^^^^^^^^^^

warning: 1 warning emitted

    Finished dev [unoptimized + debuginfo] target(s) in 2.96s
```

As a library author, adding the lint `#![deny(missing_docs)]` is a great way to
ensure the project does not drift away from being documented well, and warn is
a good way to move towards comprehensive documentation.

In our example above, the warning is resolved by adding crate level
documentation.  The `main` method does not need documentation, but it is 
present in the output.  If your main method is doing anything special, like
returning a `Result`, it could use documentation!

```rust
#![warn(missing_docs)]

//! Run program to print Hello, world.

/// Allows unhandled errors to become program return values
fn main() -> std::io::Result<()> {
    print_it();
    Ok(())
}

/// Safely prints Hello, world!
fn print_it() {
    println!("Hello, world!");
}
```
There are more lints in the upcoming chapter [Lints][rustdoc-lints].

## Examples

Of course this is contrived to be simple, but part of the power of documentation
is showing code that is easy to follow, rather than being realistic.  Docs often
shortcut error handling because often examples become complicated to follow
with all the necessary set up required for a simple example.

`Async` is a good example of this.  In order to execute an `async` example, an
executor needs to be available.  Examples will often shortcut this, and leave
users to figure out how to put the `async` code into their own runtime.

It is preferred that `unwrap()` not be used inside an example, and some of the
error handling components be hidden if they make the example too difficult to
follow.  

``````rust
/// Example
/// ```rust
/// let fourtytwo = "42".parse::<u32>()?;
/// println!("{} + 10 = {}", fourtytwo, fourtytwo+10);
/// ```
``````  

When rustdoc wraps that in a main function, it will panic due to the 
`ParseIntError` trait not implemented.  In order to help both your audience
and your test suite, this example needs to have additional work performed:

``````rust
/// Example
/// ```rust
/// # main() -> Result<(), std::num::ParseIntError> {
/// let fourtytwo = "42".parse::<u32>()?;
/// println!("{} + 10 = {}", fourtytwo, fourtytwo+10);
/// #     Ok(())
/// # }
/// ```
``````  

The example is the same on the doc page, but has that extra information
available to anyone trying to use your crate.  More about tests in the 
upcoming [Documentation tests] chapter.  

## What to Exclude

Certain parts of your public interface may be included by default in the output
of rustdoc.  The attribute `#[doc(hidden)]` hides the non-public consumption
implementation details to facilitate idiomatic Rust.  

Good examples are `impl From<Error>` for providing `?` usage, or `impl Display`.



[Documentation tests]: documentation-tests.md
[rustdoc-lints]: lints.md