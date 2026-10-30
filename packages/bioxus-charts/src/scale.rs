//! Bar geometry. Pure functions: every number a chart draws comes from here.

/// Share of the track a bar fills, in `0.0..=1.0`. Non-finite or negative values and
/// a non-positive `max` give `0.0`; a value above `max` is clamped to full.
#[must_use]
pub fn fill(value: f64, max: f64) -> f64 {
    if !value.is_finite() || !max.is_finite() || value <= 0.0 || max <= 0.0 {
        return 0.0;
    }
    (value / max).min(1.0)
}

/// Largest finite value; `0.0` for an empty slice.
#[must_use]
pub fn max_of(values: &[f64]) -> f64 {
    values
        .iter()
        .copied()
        .filter(|v| v.is_finite())
        .fold(0.0, f64::max)
}

/// SVG `points` of a line through `values` in a `width` x `height` frame. A non-finite value
/// is left out but keeps its slot on the x axis; a flat series sits mid-height.
#[must_use]
pub fn polyline(values: &[f64], width: f64, height: f64) -> String {
    let finite = || values.iter().copied().filter(|v| v.is_finite());
    let low = finite().fold(f64::INFINITY, f64::min);
    let high = finite().fold(f64::NEG_INFINITY, f64::max);
    let last = values.len().saturating_sub(1);
    let points: Vec<String> = values
        .iter()
        .enumerate()
        .filter(|(_, v)| v.is_finite())
        .map(|(i, v)| {
            let x = if last == 0 {
                0.5
            } else {
                i as f64 / last as f64
            };
            let y = if high > low {
                (high - v) / (high - low)
            } else {
                0.5
            };
            format!("{:.2},{:.2}", x * width, y * height)
        })
        .collect();
    points.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_is_proportional_and_clamped() {
        assert_eq!(fill(5.0, 10.0), 0.5);
        assert_eq!(fill(10.0, 10.0), 1.0);
        assert_eq!(fill(20.0, 10.0), 1.0);
    }

    #[test]
    fn fill_rejects_degenerate_input() {
        for (v, m) in [
            (f64::NAN, 1.0),
            (1.0, f64::NAN),
            (-1.0, 1.0),
            (1.0, 0.0),
            (1.0, -3.0),
            (f64::INFINITY, 1.0),
            (1.0, f64::INFINITY),
        ] {
            assert_eq!(fill(v, m), 0.0, "{v} / {m}");
        }
    }

    #[test]
    fn max_ignores_non_finite_values() {
        assert_eq!(max_of(&[1.0, f64::NAN, 7.5, f64::INFINITY, -2.0]), 7.5);
        assert_eq!(max_of(&[]), 0.0);
    }
}
