//! Temporary rounded-rectangle adapter while `water-rs/waterui#4434` lands.
//!
//! Emits `ShapeKind::CustomPath` unit-space commands and returns `impl View`
//! from `FilledShape::new` so `hydrolysis-m3` compiles against both `FilledShape`
//! (pre-`#4434`) and `FilledShape<S = ()>` (`waterui` `main` / `#4434`) without
//! naming `FilledShape`'s type parameter or the removed `Fixed*` / `Uneven*`
//! types. Deleted right after `#4434` lands in favor of `RoundedRectangle` with
//! `CornerRadius` / `CornerRadii`.

use core::f32::consts::{FRAC_PI_2, PI};

use waterui::View;
use waterui::color::Color;
use waterui::shape::{FilledShape, PathCommand, Shape};

/// Temporary rounded-rectangle shape emitting normalized unit-space path commands.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompatRoundedRect {
    top_leading: f32,
    top_trailing: f32,
    bottom_leading: f32,
    bottom_trailing: f32,
}

impl CompatRoundedRect {
    /// Creates a rounded rectangle with a uniform normalized corner radius.
    pub const fn uniform(radius: f32) -> Self {
        let r = clamp_fraction(radius);
        Self {
            top_leading: r,
            top_trailing: r,
            bottom_leading: r,
            bottom_trailing: r,
        }
    }

    /// Creates a rounded rectangle with independent normalized corner radii.
    pub const fn uneven(
        top_leading: f32,
        top_trailing: f32,
        bottom_leading: f32,
        bottom_trailing: f32,
    ) -> Self {
        Self {
            top_leading: clamp_fraction(top_leading),
            top_trailing: clamp_fraction(top_trailing),
            bottom_leading: clamp_fraction(bottom_leading),
            bottom_trailing: clamp_fraction(bottom_trailing),
        }
    }

    /// Fills this shape with `color`.
    pub fn fill(self, color: impl Into<Color>) -> impl View {
        FilledShape::new(self, color)
    }
}

#[inline]
const fn clamp_fraction(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 0.5)
    } else {
        0.0
    }
}

impl Shape for CompatRoundedRect {
    type Iter = [PathCommand; 10];

    fn path(&self) -> Self::Iter {
        let mut tl = self.top_leading;
        let mut tr = self.top_trailing;
        let mut br = self.bottom_trailing;
        let mut bl = self.bottom_leading;

        let mut scale = 1.0_f32;
        for sum in [tl + tr, bl + br, tl + bl, tr + br] {
            if sum > 1.0 {
                scale = scale.min(1.0 / sum);
            }
        }
        if scale < 1.0 {
            tl *= scale;
            tr *= scale;
            br *= scale;
            bl *= scale;
        }

        [
            PathCommand::MoveTo { x: tl, y: 0.0 },
            PathCommand::LineTo {
                x: 1.0 - tr,
                y: 0.0,
            },
            PathCommand::Arc {
                cx: 1.0 - tr,
                cy: tr,
                rx: tr,
                ry: tr,
                start: -FRAC_PI_2,
                sweep: FRAC_PI_2,
            },
            PathCommand::LineTo {
                x: 1.0,
                y: 1.0 - br,
            },
            PathCommand::Arc {
                cx: 1.0 - br,
                cy: 1.0 - br,
                rx: br,
                ry: br,
                start: 0.0,
                sweep: FRAC_PI_2,
            },
            PathCommand::LineTo { x: bl, y: 1.0 },
            PathCommand::Arc {
                cx: bl,
                cy: 1.0 - bl,
                rx: bl,
                ry: bl,
                start: FRAC_PI_2,
                sweep: FRAC_PI_2,
            },
            PathCommand::LineTo { x: 0.0, y: tl },
            PathCommand::Arc {
                cx: tl,
                cy: tl,
                rx: tl,
                ry: tl,
                start: PI,
                sweep: FRAC_PI_2,
            },
            PathCommand::Close,
        ]
    }
}
