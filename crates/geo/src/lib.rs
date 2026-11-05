//! Geospatial primitives will live here: CRS policy, localization, and tiling.
//! Phase 0 only establishes the crate boundary so the API never embeds ad-hoc CRS logic.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confidence {
    Verified,
    Calculated,
    Indicative,
}

impl Confidence {
    pub fn label(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Calculated => "calculated",
            Self::Indicative => "indicative",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confidence_labels_are_stable() {
        assert_eq!(Confidence::Verified.label(), "verified");
        assert_eq!(Confidence::Indicative.label(), "indicative");
    }
}
