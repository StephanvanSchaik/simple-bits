# simple-bits

The simple-bits crate provides a simple Rust trait to extract and replace bits in integer types rather than having to rely on bit shifting and masking to manipulate bits.
This crate supports `no_std` environments and has no dependencies.

Simply add the following to your `Cargo.toml` file to use this crate:

```toml
simple-bits = "1"
```

You can then use this crate as follows:

```
use simple_bits::BitsExt;

assert_eq!(0xdeadbeef_u32.extract_bits(0..16), 0xbeef);
assert_eq!(0xdeadbeef_u32.extract_bits(16..32), 0xdead);
assert_eq!(0xdeadbeef_u32.replace_bits(0..16, 0xcafe), 0xdeadcafe);
```
