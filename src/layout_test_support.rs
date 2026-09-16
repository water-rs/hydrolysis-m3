use waterui::layout::{ProposalSize, Size, StretchAxis, SubView, ViewDimensions};

pub(crate) struct FixedLeaf(pub(crate) Size);

impl SubView for FixedLeaf {
    fn measure(&self, _proposal: ProposalSize) -> ViewDimensions {
        ViewDimensions::new(self.0)
    }

    fn stretch_axis(&self) -> StretchAxis {
        StretchAxis::None
    }

    fn priority(&self) -> i32 {
        0
    }
}
