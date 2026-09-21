# Hydrolysis Material 3

`hydrolysis-m3` is the independently maintained Material 3 style for WaterUI self-drawn backends. `Material3` implements WaterUI's public widget contract and the `hydrolysis::Style` contract — pass it to the Hydrolysis runtime:

```rust
hydrolysis::run(app, hydrolysis_m3::Material3::defaults());
```

## Typeface

The theme ships Roboto's variable build in `assets/fonts/` and declares it as
a crate-local asset, so a Material app renders in Material's typeface offline,
on a machine with no font of that name installed. The whole weight axis is in
that one file, so the type scale's `Regular` and `Medium` are real instances
rather than synthesised weights.

The font is licensed under the SIL Open Font License 1.1
(`assets/fonts/OFL.txt`); that covers the font file alone, and the crate's own
code stays `MIT OR Apache-2.0`.

CJK coverage is opt-in: enable `cjk-jp`, `cjk-kr`, `cjk-sc` or `cjk-tc` to add
the matching Noto Sans CJK face, which the font cache must already hold — a
build never downloads a font.
