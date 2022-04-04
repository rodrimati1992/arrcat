[![Rust](https://github.com/rodrimati1992/arrcat/workflows/Rust/badge.svg)](https://github.com/rodrimati1992/arrcat/actions)
[![crates-io](https://img.shields.io/crates/v/arrcat.svg)](https://crates.io/crates/arrcat)
[![api-docs](https://docs.rs/arrcat/badge.svg)](https://docs.rs/arrcat/*)


Array concatenation

This crate allows concatenating multiple arrays of varying lengths into one array.

# Example

### `concat_arrays`

For more examples of using [`concat_arrays`],
you can [look here][concat_arrays_examples].

```rust
use arrcat::concat_arrays;

{
    const PRIMES: [u16; 4] = [7, 11, 13, 17];
    assert_eq!(
        concat_arrays!([3, 4, 4u16.pow(3)], PRIMES),
        [3, 4, 64, 7, 11, 13, 17],
    );
}

{

    let increasing = [8, 9, 10];

    let concated = concat_arrays!(
        // the macro can't infer the length of runtime array non-literals.
        increasing: [_; 3],
        // non-literal/path expressions need to be parenthesized.
        ([2u16, 3, 4].map(|y| x * 3)): [_; 3],
    );

    assert_eq!(concated, [8, 9, 10, 18, 27, 36]);
}

```

# No-std support

`arrcat` is `#![no_std]`, it can be used anywhere Rust can be used.

# Minimum Supported Rust Version

`arrcat` requires Rust 1.57.0, requiring crate features to use newer language features.


[concat_arrays_examples]: https://docs.rs/arrcat/*/arrcat/macro.concat_arrays.html#examples