# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1](https://github.com/water-rs/hydrolysis-m3/compare/v0.3.0...v0.3.1) - 2026-09-20

### Other

- update Cargo.toml dependencies

## [0.2.0](https://github.com/water-rs/hydrolysis-m3/compare/v0.1.1...v0.2.0) - 2026-09-11

### Added

- establish the independent Material 3 theme repository
- *(hydrolysis-m3)* re-export the Material You Argb seed type
- *(navigation)* let a destination declare how it arrives
- *(hydrolysis)* show which list row is selected
- *(controls)* [**breaking**] require a label when constructing Toggle, TextField and Picker
- *(theme)* [**breaking**] add the SelectionContainer / SelectionForeground tokens
- *(list)* [**breaking**] section headers and footers are reactive semantic text
- *(navigation)* [**breaking**] add the medium flexible app bar tier
- add the platform loading indicator as a progress style
- *(m3)* add the Material 3 button group
- *(m3)* [**breaking**] draw the stepper as a connected pair
- *(m3)* add rounded-polygon geometry for the loading indicator
- *(m3)* add the Material 3 connected button group
- *(m3)* add the Material 3 floating and docked toolbars
- *(m3)* add the Material 3 navigation rail
- *(m3)* add the Material 3 floating action button menu
- *(m3)* add the Material 3 vertical drag handle
- *(m3)* add the Material 3 split button
- *(m3)* add the Expressive FAB and icon-button size scales
- *(controls)* [**breaking**] add the Material 3 Expressive button size scale
- *(list)* drag-to-reorder, swipe-to-delete and animated index jumps
- *(testing)* see every window, not just the main one
- add locale-aware CJK and RTL support
- *(testing)* [**breaking**] quiescence-driven waits, virtual frame clock, orthogonal theme/mode, panicking interactions
- refactor navigation system
- *(controls)* end-to-end disabled state for interactive controls
- *(hydrolysis)* M3-faithful MD3 switch, smooth wheel scrolling, explicit macOS ProMotion
- *(hydrolysis)* multi-wave MD3 ripples — rapid re-presses overlap like the M3 reference

### Fixed

- *(ci)* install complete graphics prerequisites
- *(ci)* install the Linux video acceleration headers
- *(release)* close package rehearsal gaps
- *(release)* verify registry-only package graph
- *(navigation)* remove GPU readbacks and zoom fallbacks
- *(hydrolysis)* announce the picker's own label
- *(hydrolysis)* degrade a zoom transition whose geometry was never drawn
- *(navigation)* adapt ported navigation fixes to current dev APIs
- *(navigation)* dispatch retained transitions on capability, not native identity
- *(m3)* reproduce Compose's doRepeat and normalize the Material shapes
- *(hydrolysis)* stop discarding a button's custom label content
- make fill-intent frames greedy instead of merely unbounded
- *(m3)* centre the switch thumb the way Compose lays it out
- *(m3)* [**breaking**] rebuild circular indeterminate motion on Compose's animation
- *(m3)* correct body type scale and name Compose as the parity source
- *(m3)* correct state-layer opacities to Compose's StateTokens
- *(m3)* correct the radio dot and give icon buttons their touch target
- *(m3)* align button and navigation bar with Compose tokens
- *(m3)* [**breaking**] align progress indicators with Compose's Expressive tokens
- *(m3)* [**breaking**] rebuild the slider on Compose's Expressive tokens
- *(list)* animate the full approach of an indexed jump
- *(layout)* [**breaking**] stop losing a view's stretch axis when it is erased
- fix repository rule violations and refresh documentation
- *(hydrolysis)* MD3 ripple never plays backwards; align state-layer motion with the M3 reference
- *(hydrolysis)* restore interaction state layers lost in the retained-tree rewrite
- fix hydrolysis m3 picker interactions

### Other

- consume the released framework crates instead of a monorepo revision ([#2](https://github.com/water-rs/hydrolysis-m3/pull/2))
- mirror the framework's dependency patches
- certify one immutable framework revision in the test graph
- ignore standalone build artifacts
- *(canvas)* consume waterui-canvas 0.1.0 from crates.io and drop the in-tree copy
- *(release)* make workspace-internal dev-dependencies path-only
- [**breaking**] let the composition root install self-drawn realizations
- ship the licence texts in every published crate
- *(hydrolysis-m3)* sort the Argb re-export where rustfmt wants it
- prepare WaterUI 0.3 release versions
- Give the scroll region its content, and the rest of dev's red CI
- Clear the CI failures left on dev
- Format the workspace
- Merge dev into the Material 3 alignment work
- *(tests)* [**breaking**] migrate hand-written builder tests to #[waterui::test]
- Pump frames instead of sleeping in the visual acceptance tests
- Drop the now-unused controls dev-dependency and satisfy stable clippy
- Format hydrolysis_m3 sources
- State MD3 motion as tokens instead of bare millisecond literals
- Fix workspace CI failures
- Add production GPU vector map fallback
- Fix Material navigation motion and label layout
- Fix Snackbar entrance animation
- Fix Hydrolysis form text input and theme overrides
- Align Hydrolysis Material 3 with the M3 reference
- upgrade workspace dependencies
- refactor native backends and GPU surface integration
- *(backend-core)* move MD3 state-layer opacity tokens out of the engine contract
- achieve zero clippy warnings across the workspace
- clean up clippy warnings across the workspace
- SubView: Send + Sync; decouple GpuView from SubView
- hydrolysis-m3 tests: collection accessibility soundness + membership-change preservation
- Snackbar M3 stacking + entrance/exit animation; solid-circle ripple; hydrolysis ordered draw-op z-order
- M3 shape/width/animation fidelity, Material icons, fix shape rendering
- Lean dependency graph for embedded: gpu/widgets/gestures features
- Fix component label semantics
- Fix Hydrolysis example rendering and macOS acceptance
- Fix graphics lint blockers
- Fix CI typo and dependency checks
- Restore WaterUI CI gates and reactive map API
- Bundle Material CJK fallback fonts
- Fix Hydrolysis Material 3 text field and ripple animations
- Add Material elevation
- Add Material navigation view
- Add Material menu
- Add Material snackbar
- Add Material divider
- Add Material badges
- Add Material cards
- Add Material tabs
- Add Material list
- Add Material segmented buttons
- Add Material navigation drawer
- Add Material navigation bar
- Add Material dialog
- Add Material tooltips
- Add Material icon buttons
- Add Material floating action buttons
- Add Material input chip
- Add Material filter chip preview support
- Add Material suggestion chip
- Implement Material assist chip semantics
- Implement Material segmented picker style
- Use Material outlined button role colors
- Align Material button metrics with v0.192
- Install Material snackbar theme tokens
- Install Material card theme tokens
- Implement Material badge rendering for Hydrolysis
- Route Material text caret motion through theme
- Route Material navigation motion through theme
- Route Material text context menu through theme
- Route Material toggle motion through theme
- Route Material list edit metrics through theme
- Route Material tab button metrics through theme
- Route Material date picker spacing through theme
- Align Material picker menu colors
- Route Material text input caret through theme
- Route Hydrolysis dividers through Material theme
- Align Material stepper icon buttons
- Align Material data table metrics
- Align Material navigation app bar metrics
- Align Material list row metrics
- Align Material primary tabs
- Align Material picker menu tokens
- Align Material button medium tokens
- Align Material switch pressed state
- Animate Material text field focus indicator
- Align Material slider pressed handle
- Implement Hydrolysis Material motion preview foundation
- Align Material progress minimum width
- Align Material filled field shape
- Align Material radio indicator
- Align Material checkbox outline
- Align Material slider shape
- Align Material switch track outline
- Add Material You color source system
- Render Material ripple with soft edge
- Animate Hydrolysis indeterminate progress
- Align Hydrolysis M3 progress metrics
- Add Hydrolysis M3 color role tokens
- Align Hydrolysis M3 text button metrics
- Apply Material label typography to Hydrolysis buttons
- Install Material typography for Hydrolysis M3
- Drive Hydrolysis M3 chrome from Material color roles
- Add Material You color roles for Hydrolysis M3
- Add explicit Hydrolysis Material 3 preview support
- reorganize the project
