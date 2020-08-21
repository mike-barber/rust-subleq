# rust-subleq

- derived from Gary Explain's code here: https://github.com/garyexplains/examples/tree/master/compiler_test
- video reference: https://www.youtube.com/watch?v=8e7IdHG5fhQ
- some additional comments: https://www.youtube.com/post/UgxWgvnB5MmJnSOknDp4AaABCQ

Performance is better than I thought it'd be -- on par with clang, which achieved around 6.34s. The pattern-match version is slightly faster than clang with the target-cpu set in Rust; but the same might be true with clang in Windows, but I haven't spent much time trying to get it to work.

- Ryzen 3900X, stock settings, eco mode. 
- Windows 10
- Rust 1.45.2
- target-cpu=haswell
    - seems like a pretty reasonable modern "standard" 
    - nice performance uplift for the pattern-match algorithm
    - haven't had a good look why yet
- cargo run --release

```
subleq traditional iter 0 time 6.3762761s
subleq traditional iter 1 time 6.3723992s
subleq traditional iter 2 time 6.4259712s
subleq traditional iter 3 time 6.4330779s
subleq traditional iter 4 time 6.3927956s

subleq match expression iter 0 time 5.6699585s
subleq match expression iter 1 time 5.6733616s
subleq match expression iter 2 time 5.6560155s
subleq match expression iter 3 time 5.6606645s
subleq match expression iter 4 time 5.6600197s
```

