use crate::DividerMetrics;
use crate::theme::colors::MaterialColorScheme;
use cherenkov::kurbo::Rect;
use cherenkov::{Draw as _, Recorder};

pub const fn metrics() -> DividerMetrics {
    DividerMetrics::new(crate::dimensions::DIVIDER_THICKNESS)
}

pub fn draw(colors: &MaterialColorScheme, draw: &mut Recorder, bounds: Rect) {
    draw.fill(bounds, colors.outline_variant.working());
}

#[cfg(test)]
mod tests {
    use super::metrics;
    use crate::dimensions::DIVIDER_THICKNESS;

    #[test]
    fn divider_uses_material_tokens() {
        let metrics = metrics();

        assert_eq!(metrics.thickness, DIVIDER_THICKNESS);
        assert_eq!(DIVIDER_THICKNESS, 1.0);
    }
}
