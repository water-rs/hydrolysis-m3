# Material Design 3 conformance audit (issue #92)

Scope: every file under `src/`, audited against the official token sources:

- `material-components/material-web` `tokens/versions/latest` — M3 Expressive token set (v34.0.21). For components redesigned for M3 Expressive (buttons, icon buttons, FABs, segmented buttons, split buttons, navigation bar/rail, lists) this set supersedes the stale `md.comp.outlined-button` / `md.comp.navigation-bar`-era names that also ship in the same directory.
- `material-components-android` `lib/java/com/google/android/material/**/res/values/tokens.xml` at release 1.34 (`m3_comp_*` resources) — cross-checked where the web Sass is ambiguous.
- m3.material.io spec pages for anatomy and behavior where no token exists.

Status legend: `conforms` = value already matched the token; `fixed` = deviation corrected in this change; `annotated` = conforming value, token name added as a comment; `blocked` = needs a waterui capability that does not exist (details in the last section); `no token` = no applicable M3 token.

## Theme — color, elevation, typography, shape, state, motion, dimensions

| file:line | our value | spec value | token name | status |
|---|---|---|---|---|
| src/theme/colors.rs | full `MaterialColorScheme` maps 1:1 to `md.sys.color.*` | `md.sys.color.*` | `md.sys.color.*` | conforms |
| src/theme/elevation.rs | LEVEL0–LEVEL5 shadow pairs | level0–level5 | `md.sys.elevation.level*` | conforms |
| src/theme/state_layer.rs | hover 0.08 / focus 0.10 / pressed 0.10 / dragged 0.16 | same | `md.sys.state.*-state-layer-opacity` | conforms |
| src/theme/dimensions.rs:36 | `BUTTON_EXTRA_SMALL.horizontal_space` 12.0 | 8→12 | `md.comp.button.xsmall.leading-space`/`trailing-space` | fixed |
| src/theme/dimensions.rs:47 | `BUTTON_SMALL.horizontal_space` 16.0 | 16 | `md.comp.button.small.leading-space`/`trailing-space` | fixed |
| src/theme/dimensions.rs:85 | `BUTTON_EXTRA_LARGE.outline_width` 3.0 | 3 | `md.comp.button.xlarge.outlined.outline-width` | fixed |
| src/theme/dimensions.rs:203-215 | handle 4×44, pad 6, dot 4 | 4/44/6/4 | `md.comp.slider.handle.{width,height}` / `stop-indicator.*` | conforms |
| src/theme/dimensions.rs:215 | `SLIDER_STOP_INDICATOR_END_SPACE` 4.0 | 4 | `md.comp.slider.stop-indicator.trailing-space` | fixed (added) |
| src/theme/typography.rs:46-96 | label small/medium/large; +`label_medium_prominent`, `label_large_prominent` (Bold weight) | label scale incl. `prominent` weights | `md.sys.typescale.label-*` | conforms (helpers added for `*.label-text.weight` tokens) |
| src/theme/motion/* | spring/duration scheme | — | `md.sys.motion.*` | conforms |
| src/semantics.rs | `conditional_font` added (mirrors `conditional_color`) | — | supports `*.active.label-text.weight` | fixed (added) |

## Buttons (`src/controls/button.rs`, `src/theme/dimensions.rs`)

| file:line | our value | spec value | token name | status |
|---|---|---|---|---|
| src/controls/button.rs:164-172 | filled disabled container = on-surface @ 0.1 | 0.12→0.10 | `md.comp.button.filled.disabled.container.opacity` | fixed |
| src/controls/button.rs:58 | outlined label/icon/layer = on-surface-variant | was primary | `md.comp.button.outlined.{label-text,icon,*.state-layer}.color` | fixed |
| src/controls/button.rs:60 | text button label/layer = primary | primary | `md.comp.button.text.label-text.color` | conforms |
| src/controls/button.rs:517-537 | outlined border = outline-variant, width 1/1/1/2/3 per size | was outline, flat 1 | `md.comp.button.outlined.outline.color` + `md.comp.button.<size>.outlined.outline-width` | fixed |
| src/controls/button.rs:101-115 | pressed corner morph 8/8/12/16/16 per size | new | `md.comp.button.<size>.pressed.container.shape` | fixed (driven by `state.press_waves.latest().progress`) |
| src/controls/button.rs:398-399 | disabled outlined border stays outline-variant | outline-variant | `md.comp.button.outlined.disabled.outline.color` | conforms |
| container heights | 32/40/48/56/64 | 32/40/48/56/64 | `md.comp.button.<size>.container.height` | conforms |
| label font | label-large at every size | label-large | `md.comp.button.<size>.label-text` | conforms |

## Icon buttons (`src/icon_button.rs`)

| file:line | our value | spec value | token name | status |
|---|---|---|---|---|
| src/icon_button.rs:49-96 | five sizes xs/s/m/l/xl: layer 32/40/56/96/136, icon 20/24/24/32/40, outline 1/1/1/2/3 | added xs + xl | `md.comp.icon-button.<size>.*` | fixed |
| src/icon_button.rs:183-184 | filled container primary; disabled container opacity 0.1 | 0.12→0.10 | `md.comp.icon-button.filled.disabled.container.opacity` | fixed |
| src/icon_button.rs:193-195 | outlined border outline-variant in all states | was outline | `md.comp.icon-button.outlined.{outline,disabled.outline}.color` | fixed |
| src/icon_button.rs:228-230 | layer on-primary (filled) / on-surface-variant (standard, outlined) | same | `md.comp.icon-button.*.state-layer.color` | conforms |
| src/icon_button.rs:250-264 | pressed corner morph 8/8/12/16/16 | new | `md.comp.icon-button.<size>.pressed.container.shape` | fixed |
| pressed morph on custom interaction style | — | per-state shape | `md.comp.icon-button.<size>.pressed.container.shape` | blocked: morph implemented via binding-driven shape; per-state radii on `InteractionStyle` still unavailable for composed styles |

## FABs (`src/fab.rs`, `src/fab_menu.rs`)

| file:line | our value | spec value | token name | status |
|---|---|---|---|---|
| src/fab.rs:53-95 | sizes FAB 56/corner-16/icon-24, medium 80/corner-20/icon-28, large 96/corner-28/icon-36 | 56/80/96 | `md.comp.fab{,.medium,.large}.container.*` | fixed; `md.comp.fab.small` (40dp) dropped — deprecated in M3 Expressive (MDC-Android `FloatingActionButton.md`: "Deprecated small FAB size"; `floatingactionbutton/attrs.xml:28` retires `fabSize`); `SurfaceFab` kept (`md.comp.fab-surface` + MDC styles still ship, though Expressive no longer recommends it) |
| src/fab.rs | container primary-container, icon on-primary-container | same | `md.comp.fab.container.color` / `icon.color` | conforms |
| src/fab.rs | container elevation level3, hover level4 | level3; hover L4 | `md.comp.fab.{container,hover.container}.elevation` | partially: resting L3 conforms; hover→L4 blocked (no per-state elevation on `FloatingStyle`) |
| src/fab_menu.rs:236-254 | items h56, corner-full, level0, primary-container/on-primary-container, icon 24, label large | level3→level0 | `md.comp.fab-menu.menu-item.*` | fixed |
| src/fab_menu.rs:258-273 | close button 56×56, corner-full, icon 20, level3, primary/on-primary | was primary-container | `md.comp.fab-menu.primary.close-button.*` | fixed |
| fab-menu item per-state elevation | — | level0 → higher on press | `md.comp.fab-menu.menu-item.*.container.elevation` | blocked: no per-state elevation |

## Button group & segmented & split buttons (`src/button_group.rs`, `src/segmented_button.rs`, `src/split_button.rs`, `src/controls/picker.rs`)

| file:line | our value | spec value | token name | status |
|---|---|---|---|---|
| src/button_group.rs:107-120 | resting radii per position (outer=outer corner, inner=inner corner) | segment shape | `md.comp.button-group.*` | fixed (`interaction_style_with_radii` at :266) |
| src/segmented_button.rs | selected secondary-container/on-secondary-container; segment layer on-secondary-container | same | `md.comp.segmented-button.*` | conforms |
| src/split_button.rs:47,200-214 | pressed half softens seam corner to 12 (corner-medium) | new | `md.comp.split-button.*.pressed.inner-corner-radius` | fixed (binding-driven `UnevenRoundedRectangle` morph) |
| src/split_button.rs:219 | label label-large | label-large | `md.comp.split-button.label-text` | fixed |
| src/split_button.rs:246-290 | state layer clipped to each half's radii | half-shape | `md.comp.split-button.*.state-layer` | fixed (`interaction_style_with_radii`) |
| split-button hovered morph | — | inner-corner corner-medium on hover | `md.comp.split-button.*.hovered.*` | blocked: no hover signal per half; pressed morph implemented only |

## Chips (`src/chip.rs`)

| file:line | our value | spec value | token name | status |
|---|---|---|---|---|
| src/chip.rs:120-124 | flat chips transparent fill, outline-variant border | was elevated/tinted | `md.comp.assist-chip.flat.*` | fixed |
| src/chip.rs:285-292 | filter selected → secondary-container fill, outline width 0 | was outline kept | `md.comp.filter-chip.flat.selected.*` | fixed |
| src/chip.rs:396 | input-chip flat outline outline-variant | outline-variant | `md.comp.input-chip.flat.unselected.outline.color` | conforms |
| chip focus outline | — | per-state outline color | `md.comp.*-chip.flat.focused.outline.color` | blocked: no per-state content/outline color on `InteractionStyle` |

## Dialog, tooltip, drag handle, badge (`src/dialog.rs`, `src/tooltip.rs`, `src/drag_handle.rs`, `src/layout/badge.rs`, `src/material_badge.rs`)

| file:line | our value | spec value | token name | status |
|---|---|---|---|---|
| src/dialog.rs:223 | container elevation level3 | level3 | `md.comp.dialog.container.elevation` | conforms |
| src/dialog.rs | corner-extra-large (28), padding 24, headline headline-small, body body-medium | same | `md.comp.dialog.*` | conforms |
| src/tooltip.rs:397 | supporting tooltip elevation level2, corner 4, plain tooltip inverse-surface | same | `md.comp.tooltip.*` | conforms |
| src/layout/badge.rs:19-28 | badge container error, label on-error, sizes 6/16 | same | `md.comp.badge.*` | conforms |
| src/drag_handle.rs | drag-handle 4×22 corner-full on-surface-variant@0.4 | same | `md.comp.drag-handle.*` | conforms |

## Controls (`src/controls/*`)

| file:line | our value | spec value | token name | status |
|---|---|---|---|---|
| src/controls/input.rs:107-127 | filled text field draws hover-only state layer on-surface@0.08 clipped to top-rounded (4,4,0,0) | hover layer only | `md.comp.filled-text-field.hover.state-layer.{color,opacity}` | fixed |
| src/controls/input.rs:61-76 | container surface-container-highest, caret/indicator per state | same | `md.comp.filled-text-field.*` | conforms |
| src/controls/picker.rs:108 | select-field layer clipped to (4,4,0,0) | top-rounded | `md.comp.filled-select.text-field.container.shape` | fixed |
| src/controls/picker.rs:176 | menu row layer on-surface (all states) | on-surface | `md.comp.filled-select.menu.list-item.*.state-layer.color` | fixed |
| src/controls/picker.rs:185 | menu separator surface-variant | was outline-variant | `md.comp.filled-select.menu.divider.color` | fixed |
| src/controls/picker.rs | selected menu row surface-container-highest | surface-container-highest | `md.comp.filled-select.menu.list-item.selected.container.color` | conforms |
| src/controls/picker.rs:340-349 | segmented layer on-secondary-container (selected) | on-secondary-container | `md.comp.segmented-button.selected.*.state-layer.color` | conforms |
| src/controls/progress.rs | track/active-indicator heights, gap, shapes per expressive progress | same | `md.comp.{linear,circular}-progress-indicator.*` | conforms |
| src/controls/slider.rs:147 | handle elevation level1 (disabled level0) | was level0 | `md.comp.slider.handle.elevation` / `disabled.handle.elevation` | fixed |
| src/controls/slider.rs:83-103 | single stop dot at the track's trailing end, offset = trailing-space 4 + r2, on-secondary-container | trailing only | `md.comp.slider.{stop-indicator.trailing-space,inactive.stop-indicator.container.color}` | fixed — leading dot removed: no `leading-space`/`leading stop-indicator` token exists; Compose `SliderDefaults.TrackStopIndicatorSize` is "at the end of the track" and the M3 accessibility page places the dot at the end of the inactive track; MDC-Android's `trackStopIndicatorSize` doc says "edges" but is one attr shared across Slider/RangeSlider/CenteredSlider |
| src/controls/slider.rs:160-170 | unbounded 40dp halo, primary, hover/focus/press opacities | new | `md.comp.slider.state-layer.size/color` | fixed |
| src/controls/slider.rs | track 16h, handle 4×44, gap 6 | same | `md.comp.slider.*` | conforms |
| src/controls/slider size variants + value label | — | small/medium/large, value indicator | `md.comp.slider.*-size` / `value-indicator.*` | blocked: single-size `MaterialSliderStyle` only; no value-label surface in waterui slider |
| src/controls/toggle.rs:127-142 | thumb shadow level1 enabled / level0 disabled | was none | `md.comp.switch.handle.elevation` / `disabled.handle.elevation` | fixed |
| src/controls/toggle.rs | track 52×32, handle 16→28 grow, icon 16; handle color primary-container/on-surface-variant on hover+focus+pressed | same | `md.comp.switch.*` | conforms (color gate kept — tokens give same color on all three states) |
| src/controls/stepper.rs | stepper chrome | — | no M3 stepper component | no token |

## Lists (`src/material_list.rs`, `src/layout/list.rs`)

| file:line | our value | spec value | token name | status |
|---|---|---|---|---|
| src/material_list.rs:25-27 | leading/trailing icons 20×20 | was 24 | `md.comp.list.list-item.*-icon.expressive.size` | fixed |
| src/material_list.rs:222 | interactive item layer radius corner-extra-small (8), on-surface | was 0 | `md.comp.list.list-item.container.expressive.shape` + `*.state-layer.color` | fixed |
| src/material_list.rs | one-line 56 / two-line 72, body-large headline, body-medium supporting, label-small trailing | same | `md.comp.list.list-item.*` | conforms |
| src/material_list.rs | selected container secondary-container | secondary-container | `md.comp.list.list-item.selected.container.color` | blocked: `MaterialListItem` has no `selected` API |
| src/layout/list.rs:155-161 | reorder drag: level4 shadow + tertiary-container + corner-large | added | `md.comp.list.reorder.list-item.*` | fixed |
| src/layout/list.rs:167 | row separator outline 1px | was outline-variant | `md.comp.list.divider.color` | fixed |
| list-item shape morph by state | — | rest 8 / hover 12 / pressed+focused+dragged 16 | `md.comp.list.list-item.container.*-state.expressive.shape` | blocked: `InteractionStyle` has no per-state corner radius |

## Menus (`src/layout/menu.rs`, `src/material_menu.rs`)

| file:line | our value | spec value | token name | status |
|---|---|---|---|---|
| src/layout/menu.rs:34 | context panel level2, surface-container, corner 4 | same | `md.comp.menu.container.*` | conforms |
| src/layout/menu.rs:48 | separator surface-variant 1px | was outline-variant | `md.comp.menu.divider.color` | fixed |
| src/material_menu.rs | row 48h, padding 12/8, width 112–280 | same | `md.comp.menu.list-item.*` | conforms |

## Cards, snackbar, divider, table, scroll (`src/layout/*`, `src/material_*`)

| file:line | our value | spec value | token name | status |
|---|---|---|---|---|
| src/layout/card.rs:8-11,63-85 | corner 12; elevated L1 / filled L0 / outlined 1px border | same | `md.comp.{elevated,filled,outlined}-card.*` | conforms |
| src/layout/snackbar.rs:11-29 | 48h single-line, pad 16/12, 288–568w, corner 4, level3 | same | `md.comp.snackbar.*` | conforms |
| src/layout/snackbar.rs | container inverse-surface, label inverse-on-surface, action inverse-primary | same | `md.comp.snackbar.*` | conforms |
| src/layout/divider.rs:10 | divider outline-variant 1px | outline-variant | `md.comp.divider.color` | conforms |
| src/layout/table.rs:21-49 | surface rows, outline-variant gridlines, cell pad 32 | same | `md.comp.data-table.*` | conforms |
| src/layout/scroll.rs | scroll chrome | — | no M3 scrollbar tokens | no token |

## Navigation (`src/navigation_bar.rs`, `src/navigation_rail.rs`, `src/navigation_drawer.rs`, `src/navigation/tabs.rs`, `src/navigation/navigation.rs`, `src/material_navigation.rs`, `src/material_tabs.rs`, `src/toolbar.rs`)

| file:line | our value | spec value | token name | status |
|---|---|---|---|---|
| src/navigation_bar.rs:146 | item layer on-secondary-container, both states | was on-surface (inactive) | `md.comp.nav-bar.item.*.state-layer.color` | fixed |
| src/navigation_bar.rs:263 | active indicator 56×32 capsule secondary-container | was 64w | `md.comp.nav-bar.item.vertical.active-indicator.{width,height}` | fixed |
| src/navigation_bar.rs | bar h64, active label secondary + label-medium-prominent weight | added prominent weight | `md.comp.nav-bar.*` | fixed (weight) / conforms (rest) |
| src/navigation_bar.rs | horizontal item variant (h40 indicator) | h40 | `md.comp.nav-bar.item.horizontal.*` | blocked: only vertical layout supported |
| src/navigation_rail.rs:255-259 | active label secondary; layer on-secondary-container both states | was primary/on-surface | `md.comp.nav-rail.item.*` | fixed |
| src/navigation_rail.rs:40-63,294-300 | expanded item 64h with 56h horizontal indicator; collapsed 56h item + 56×32 pill; expanded label label-large(-prominent) | was 32h pill | `md.comp.nav-rail.{collapsed,expanded}.*` | fixed |
| src/navigation_drawer.rs:301-308 | active layer on-secondary-container; inactive layer on-surface; active label label-large-prominent | added prominent weight | `md.comp.navigation-drawer.*` | fixed |
| src/navigation_drawer.rs | inactive pressed layer on-secondary-container (vs on-surface for hover/focus) | split per state | `md.comp.navigation-drawer.inactive.*.state-layer.color` | partially: on-surface used for all inactive states — `InteractionStyle` cannot split hover/focus from pressed color |
| src/navigation/tabs.rs:33 | tab strip separator surface-variant | was outline | `md.comp.primary-navigation-tab.divider.color` | fixed |
| src/navigation/navigation.rs | app-bar separator removed | none | `md.comp.app-bar` defines no divider token | fixed (separator now a no-op) |
| src/toolbar.rs | toolbar row | — | `md.comp.toolbar.*` exists (docked/floating) | no token implemented: file exposes plain row layout, no M3 chrome claimed |
| app-bar scrolled elevation | — | on-scroll L2 + surface-container | `md.comp.app-bar.scrolled.*` | blocked: no scroll-state signal to `MaterialNavigation` |

## Misc (`src/material_*`, `src/icons.rs`, `src/icon_paths.rs`, `src/lib.rs`)

| file:line | our value | spec value | token name | status |
|---|---|---|---|---|
| src/material_card.rs / material_divider.rs / material_snackbar.rs / material_tabs.rs | public views delegate to the audited layout/navigation drawers | — | — | conforms |
| src/icons.rs / icon_paths.rs | icon geometry | — | no tokens | no token |
| src/lib.rs | exports | — | — | no token |

## Blocked on missing waterui capabilities (reported, not approximated)

1. **Per-state corner radius on `InteractionStyle`** — needed for list-item shape morph (8→12→16 by hover/pressed/focused/dragged) and any composed per-state shape on segmented/icon-button containers. Currently only the resting radius can be installed.
2. **Per-state elevation on `FloatingStyle`** — FAB hover L3→L4, fab-menu item states, and `md.comp.elevated-card.hovered.elevation`.
3. **Per-state content/outline color on `InteractionStyle`** — nav-drawer inactive pressed layer (on-secondary-container) vs hover/focus (on-surface); chip focus outline color.
4. **Hover signal on individual split-button halves** — pressed inner-corner morph implemented via drag bindings; the hovered-corner morph has no signal source.
5. **`selected` flag on `MaterialListItem` / plain-button APIs** — `md.comp.list.list-item.selected.container.color` and selected state layers unreachable.
6. **Horizontal navigation-bar item variant** — `md.comp.nav-bar.item.horizontal.*` (h40 full-width indicator) has no layout slot.
7. **Slider size variants and value indicator** — `md.comp.slider.{small,large}.*` and `md.comp.slider.value-indicator.*`; the style is single-size and there is no value-label surface.
8. **App-bar scrolled state** — `md.comp.app-bar.scrolled.container.elevation/color` needs a scroll-position signal.
9. **Focus indicator ring** — `md.comp.focus-indicator.*` (secondary, 3px, offset) needs a focus-ring draw callback that does not exist.

## Tests updated

- `src/controls/button.rs` unit tests: disabled filled container opacity 0.1; outlined outline-variant + banded widths; pressed corner morph.
- `src/icon_button.rs` unit tests: xs/xl size map, outline width table.
- `src/fab.rs` unit test: `FabSize` = Baseline 56/Medium 80/Large 96 only (no `Small`).
- `src/controls/picker.rs`: `menu_selected_row_and_divider_use_filled_select_tokens` → surface-variant divider.
- `src/navigation_bar.rs`: indicator width 56.
- `src/navigation_rail.rs`: expanded indicator height 56.
- `src/material_list.rs`: icon sizes 20.
- `src/controls/slider.rs`: handle-level shadow + trailing stop dot + halo assertions.
- Visual acceptance tests (`*_visual.rs`, ignored by default) regenerate their PNGs under the new tokens; they are opt-in rendering comparisons, not CI assertions.
