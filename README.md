# optimus-rs

Optimus id obfuscation / transformation library for rust.

## Library

```rust
fn main() {
    
    // First number must be large PRIME number lower than i32::MAX
    // Second number is modular inverse of first number with (i32::MAX as u64 + 1)
    // Third number is large random number lower than i32::MAX
    
    let enc = Optimus::new(1580030173, Some(59260789), Some(1163945558)).unwrap();
    let new_id = enc.encode(15);
    assert_eq!(new_id, 1103647397);
    let orig_id = enc.decode(new_id);
    assert_eq!(orig_id, 15);
    
    // or you can omit second and third arguments to auto create them
    // don't forget to decode encoded id you have to use all the same 3 numbers
}
```

## Caveats

Here you can find archives with prime numbers
[primes](https://primes.utm.edu/lists/small/millions/).

## Reference

- [optimus](https://github.com/jenssegers/optimus)
- [optimus-go](https://github.com/pjebs/optimus-go)
