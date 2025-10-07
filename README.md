# rust-native-obf

advanced native obfuscation library for rust

## features

**compile time obfuscation**
- string encryption with pcg keystream
- byte array obfuscation
- wide string (utf-16) obfuscation
- compile time random number generation
- compile time hashing (xxhash32, sdbm)
- constant value encoding with mba

**runtime obfuscation**
- control flow obfuscation
- opaque predicates
- hidden function calls
- obfuscated conditionals
- cascade encryption
- stack trashing

**advanced techniques**
- static reference obfuscation
- pointer mangling
- obfuscated value storage
- anti-debugging checks
- tamper detection
- noise generation

## usage

```toml
[dependencies]
rust-native-obf = "0.1.0"
```

### string obfuscation

```rust
use rust_native_obf::*;

let secret = obf_str!("my secret key");
let bytes = obf_bytes!(b"secret data");
let wide = obf_wide!("wide string");
```

### compile time features

```rust
let hash = ct_xxhash!(b"data");
let random = ct_rand!(u32);
let encoded = obf_const!(42, u32);
```

### control flow

```rust
let result = hidden_call!({
    sensitive_function()
});

obf_if!(condition, {
    do_something();
}, {
    do_else();
});
```

### advanced obfuscation

```rust
static DATA: [u8; 4] = [1, 2, 3, 4];
let obf_ref = obf_static_ref!(&DATA);

let obf_val = ObfuscatedValue::new(123456);
let original = obf_val.get();

let encrypted = cascade_encrypt(data, 5);
```

### anti-analysis

```rust
debug_trap!();

if !anti_debug() {
    panic!("debugger detected");
}
```

## examples

```bash
cargo run --example basic
```

## how it works

**string obfuscation** - uses pcg random number generator for keystream generation, encrypts at compile time, decrypts with volatile reads to prevent constant folding

**control flow** - generates unique keys for each code block based on statement content, executes in randomized order

**value encoding** - uses mixed boolean arithmetic (mba) with rotation and multiplication for reversible encoding

**pointer obfuscation** - mangles static references using xxhash-based transformation with black_box to prevent llvm optimization

**compile time rng** - combines file location, line number, column with xxhash mixing and siphash for deterministic randomness

**anti-debug** - checks for debugger presence on windows, can be extended for other platforms

## technical details

- no_std compatible core modules
- volatile memory operations
- black_box for compiler barrier
- const fn for compile time execution
- zero runtime overhead for constants

## notes

this is obfuscation, not cryptography. designed to make reverse engineering harder, not impossible. do not use for hiding actual secrets in client binaries.

## license

MIT
