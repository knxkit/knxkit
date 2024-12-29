Pre-generated datapoint definitions for [knxkit](https://crates.io/crates/knxkit)
=

#### Exmple

```rust
use knxkit_dpt::specific::DPT_4_7;

let dp = DPT_4_7 {
    Increase: true,
    StepCode: 7,
}.to_data_point();
```

