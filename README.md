# Hydrolysis Material 3

`hydrolysis-m3` is the independently maintained Material 3 style for WaterUI self-drawn backends. `Material3` implements WaterUI's public widget contract and the `hydrolysis::Style` contract — pass it to the Hydrolysis runtime:

```rust
hydrolysis::run(app, hydrolysis_m3::Material3::defaults());
```
