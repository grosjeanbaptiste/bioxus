//! Form factors by viewport width. One rule for web, desktop, tablet and phone:
//! the width decides, not the platform. Breakpoints are in `rem`, as in the CSS.

/// Minimum width, in rem, from which a viewport counts as a tablet.
pub const TABLET_MIN_REM: f32 = 40.0;
/// Minimum width, in rem, from which a viewport counts as a desktop.
pub const DESKTOP_MIN_REM: f32 = 64.0;

/// The layout class of a viewport.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormFactor {
    Phone,
    Tablet,
    Desktop,
}

impl FormFactor {
    /// Classify a viewport `width_rem` wide. A NaN or negative width is a phone,
    /// the smallest layout, which every screen can show.
    #[must_use]
    pub fn from_width_rem(width_rem: f32) -> Self {
        if width_rem >= DESKTOP_MIN_REM {
            Self::Desktop
        } else if width_rem >= TABLET_MIN_REM {
            Self::Tablet
        } else {
            Self::Phone
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundaries_belong_to_the_larger_class() {
        assert_eq!(FormFactor::from_width_rem(39.99), FormFactor::Phone);
        assert_eq!(FormFactor::from_width_rem(40.0), FormFactor::Tablet);
        assert_eq!(FormFactor::from_width_rem(63.99), FormFactor::Tablet);
        assert_eq!(FormFactor::from_width_rem(64.0), FormFactor::Desktop);
    }

    #[test]
    fn degenerate_widths_fall_back_to_phone() {
        for w in [f32::NAN, -1.0, 0.0] {
            assert_eq!(FormFactor::from_width_rem(w), FormFactor::Phone);
        }
        assert_eq!(
            FormFactor::from_width_rem(f32::INFINITY),
            FormFactor::Desktop
        );
    }

    #[test]
    fn css_breakpoints_match_the_constants() {
        let css = crate::STYLESHEET;
        assert!(css.contains(&format!("min-width: {TABLET_MIN_REM}rem")));
        assert!(css.contains(&format!("min-width: {DESKTOP_MIN_REM}rem")));
    }
}
