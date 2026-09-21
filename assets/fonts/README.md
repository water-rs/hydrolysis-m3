# Bundled fonts

## Roboto

`Roboto-Variable.ttf` is Roboto's variable build, carrying the `wght`
(100–900) and `wdth` axes in one file, so the Material type scale's
`Regular` (400) and `Medium` (500) come from real instances of the axis
rather than a synthesised weight.

- Upstream: <https://github.com/google/fonts>, `ofl/roboto/Roboto[wdth,wght].ttf`
- Renamed here only to keep the path free of `[`, `]` and `,`.
- Licence: SIL Open Font License 1.1 — `OFL.txt` in this directory. That
  licence covers the font file alone; the crate's own code stays
  `MIT OR Apache-2.0`.

The theme ships it in-tree rather than declaring it remotely because a
build must never reach the network: a Material app has to render in
Material's typeface on a machine that has never seen it, offline, with no
system font of that name.
