# eytzinger search

A cache-friendly binary search implementation using the Eytzinger layout.

```rust
fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let eyt = EytzingerVec::from_slice(&data);

    let res = eyt.search(8);
    assert_eq!(res, 1); // returns the index of element

    let res = eyt.search(69);
    assert_eq!(res, 0); // returns 0 if not found
}
```

## Prefetching

This implementation makes use of `core_intrinsics::prefetch_read_data` to work across different targets. Temporal
locality is set to `3`, indicating maximum temporal locality.

## Memory Alignment

`AlignedVec` holds the `#[repr(align(64))]` attribute. This ensures the vector is aligned to 64-byte cache lines,
optimizing memory access patterns.

## Literature

* **https://algorithmica.org/en/eytzinger**
* https://arxiv.org/pdf/1509.05053





