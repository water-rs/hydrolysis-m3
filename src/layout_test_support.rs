use waterui::layout::{
    LayoutError, ProposalSize, Rect, ResolvedExtent, Size, StretchAxis, SubView, ViewDimensions,
};

pub struct FixedLeaf(pub Size);

impl SubView for FixedLeaf {
    fn measure(&self, _proposal: ProposalSize) -> ViewDimensions {
        ViewDimensions::new(self.0)
    }

    fn resolve_extent(
        &self,
        bounds: Rect,
        proposal: ProposalSize,
    ) -> Result<ResolvedExtent, LayoutError> {
        ResolvedExtent::centred(bounds, self.measure(proposal).size)
    }

    fn stretch_axis(&self) -> StretchAxis {
        StretchAxis::None
    }

    fn priority(&self) -> i32 {
        0
    }
}
