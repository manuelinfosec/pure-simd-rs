# pure-simd-rs
Writing SIMD-accelerated code in pure Rust; tested on M4 Pro &amp; AMD Zen 5 CPUs.

SIMD stands for Single Instruction, Multiple Data: CPU instructions that can operate on larget data vectors.

```rust
// instead of doing this
let mut a = [1,2,3,4];
for n in &a {
    *n += 10;
}

// do this
let mut vector = u64x4::from_array([1,2,3,4]); // a 256-bit vector of 4 unit64
let x = u64x64::splat(10); // create a 256-bit vector of 4 unit53 (10, 10, 10, 10)
let vector = vector + x;
// vector = u64x64(11, 12, 13, 4);
```

SIMD involves: 

**load -> compute -> store**

There are generally two ways to accelerate algorithms with SIMD instructions. 

The first way is to find operations that can be performed in parrallel for your algorithm, but it's algorithm-specific and often more complex to implement.

The second way, generic and easier to implement is to "split" your input into chunks that each contain `X` blocks of data, where `X` is the number of available lanes you can compute the `X` blocks in parrallel. 