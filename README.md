# `num-ordinal`: ordinal number types in Rust

[Documentation](https://docs.rs/num-ordinal)

Ordinal numbers (_first, second, third, ..._) are usually represented
as 0-based or 1-based integers. In English and most other natural
languages, they're represented as 1-based numbers:
first = 1st, second = 2nd, third = 3rd and so on.
However, most programming languages are zero-based, i.e. when getting
the first element in array or list, the index is 0. This is also true for Rust.

## Usage

To make working with ordinal numbers more explicit and less error-prone,
this library provides ordinal number types that can be converted to/from
cardinal numbers while specifying if it is 0- or 1-based:

```rust
use num_ordinal::{Ordinal, Osize};

// Osize is an ordinal usize
let o = Osize::from0(3);
assert_eq!(&o.to_string(), "4th");

let o = Osize::from1(3);
assert_eq!(&o.to_string(), "third");
```

There are also two convenience functions to create ordinal numbers
when the return type can be inferred:

```rust
use num_ordinal::{Osize, ordinal0, ordinal1};

// Osize is an ordinal usize
let o: Osize = ordinal0(3);
assert_eq!(&o.to_string(), "4th");

let o: Osize = ordinal1(3);
assert_eq!(&o.to_string(), "third");
```

And [a macro](ordinal):

```rust
use num_ordinal::{O32, ordinal};

// type is inferred:
let o: O32 = ordinal!(4-th);

// type can also be specified:
let o = ordinal!(4-th O32);
```

## Implemented traits

Ordinal numbers implement a number of traits, so they can be
compared, hashed, copied and formatted. Also, you can add or
subtract an integer from an ordinal number:

```rust
use num_ordinal::ordinal;

assert_eq!(ordinal!(5-th O32) - 3, ordinal!(second O32));
```

Subtracting an ordinal from an ordinal produces an integer:

```rust
use num_ordinal::ordinal;

assert_eq!(ordinal!(5-th O32) - ordinal!(second O32), 3);
```

The default value is _first_.

## Features

* `serde`: Implement `Serialize` and `Deserialize` for ordinals

## License

MIT
