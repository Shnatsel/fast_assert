# A faster `assert!` for Rust

A drop-in replacement for the standard library's [`assert!`](https://doc.rust-lang.org/stable/std/macro.assert.html)
macro that emits far less code in the hot path, where the assertion holds.

This reduces instruction cache pressure,
and may allow for more optimizations by the compiler due to more aggressive inlining of hot functions.

`fast_assert!` only adds two extra instructions for the default error message
and three instructions for a custom error message into the hot path,
while the standard library's `assert!` adds five instructions for the default error message
in the hot path and lots for a custom error message.

## How?

We defer all the work that needs to be done in case of a panic, such as formatting the arguments,
to separate functions annotated with `#[cold]`. That way the function that calls `fast_assert!`
can stay as lean as possible.

By comparison, the std `assert!` emits some of the code executed only in case of a panic
inside the function that invokes it. Even if that code isn't executed, you still pay a (small) price
for it being present inside your hot function.

## Why not improve the standard library instead?

The standard library's `assert!` is implemented not as a macro, but as a compiler built-in,
which makes it difficult to modify and contribute to.

Improving the default message codepath should be fairly straightforward for someone familiar
with Rust compiler internals, and I hope this crate inspires such as pull request!

The custom message codepath is trickier. We use a closure to defer all argument formatting
to a separate cold function. This works identically to std `assert!` the vast majority of the time,
but there might be some edge cases in which it would break, so such a change might not be acceptable
for the standard library, at least outside a new language edition. Or maybe it's fine - who knows?

Please take this as an invitation to improve the std `assert!` and make this crate obsolete!