# How to write documentation

Good documentation is not natural.  There are opposing forces that make good
documentation difficult.  It requires expertise in the subject but also
requires writing to a novice perspective.  Documentation therefore often 
glazes over implementation detail, or leaves an explain like I'm 5 response.

There are tenants to Rust documentation that can help guide anyone through
the process of documenting libraries so everyone has ample opportunity to
use the code.  

This chapter covers not only how to write documentation but specifically
how to write **good** documentation.  It is important to be as clear
as you can, and as complete as possible.  As a rule of thumb: the more
documentation you write for your crate the better.  If an item is public
then it should be documented.

## Getting Started

Documenting a crate should begin with front-page documentation.  As an
example, [hashbrown](https://docs.rs/hashbrown/0.8.2/hashbrown/) crate level
documentation summarizes the role of the crate, provides links to explain
technical details, and gives the reason why to use the crate.  

After introducing the crate, it is important that within the front-page 
an example be given how to use the crate in a real world setting.  The
example benefits from isolating the library's role from the implementation
details, but doing so without shortcuts also benefits users who may copy
and paste the example to get started. 

[futures](https://docs.rs/futures/0.3.5/futures/) uses an approach of using
inline comments to explain line by line the complexities of using a future,
because often people's first exposure to rust's future is this example.

[backtrace](https://docs.rs/backtrace/0.3.50/backtrace/) usage walks through
the whole process, explaining changes made to the `Cargo.toml` file, passing
command line arguments to the compiler, and shows a quick example of
backtrace in the wild.  

Finally, the front-page can eventually become a comprehensive reference
how to ues a crate, like the usage found in 
[regex](https://docs.rs/regex/1.3.9/regex/).  In this front page, all the 
requirements are outlined, the gotchas are taught, then practical examples
are provided.  The front page goes on to show how to use regular expressions
then concludes with crate features.

Don't worry about comparing your crate that is just beginning to get
documentation to something more polished, just start incrementally and put
in an introduction, example, and features.  Rome wasn't built in a day!

## Basic structure

It is recommended that each item's documentation follows this basic structure:

```text
[short sentence explaining what it is]

[more detailed explanation]

[at least one code example that users can copy/paste to try it]

[even more advanced explanations if necessary]
```

This basic structure should be straightforward to follow when writing your
documentation and, while you might think that a code example is trivial,
the examples are really important because they can help your users to
understand what an item is, how it is used, and for what purpose it exists.

Let's see an example coming from the [standard library] by taking a look at the
[`std::env::args()`][env::args] function:

``````text
Returns the arguments which this program was started with (normally passed
via the command line).

The first element is traditionally the path of the executable, but it can be
set to arbitrary text, and may not even exist. This means this property should
not be relied upon for security purposes.

On Unix systems shell usually expands unquoted arguments with glob patterns
(such as `*` and `?`). On Windows this is not done, and such arguments are
passed as-is.

# Panics

The returned iterator will panic during iteration if any argument to the
process is not valid unicode. If this is not desired,
use the [`args_os`] function instead.

# Examples

```
use std::env;

// Prints each argument on a separate line
for argument in env::args() {
    println!("{}", argument);
}
```

[`args_os`]: ./fn.args_os.html
``````

As you can see, it follows the structure detailed above: it starts with a short
sentence explaining what the functions does, then it provides more information
and finally provides a code example.

## Markdown

`rustdoc` is using the [commonmark markdown specification]. You might be
interested into taking a look at their website to see what's possible to do.

## Lints

To be sure that you didn't miss any item without documentation or code examples,
you can take a look at the rustdoc lints [here][rustdoc-lints].

[standard library]: https://doc.rust-lang.org/stable/std/index.html
[env::args]: https://doc.rust-lang.org/stable/std/env/fn.args.html
[commonmark markdown specification]: https://commonmark.org/
[rustdoc-lints]: lints.md
