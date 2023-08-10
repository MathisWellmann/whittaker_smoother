use crate::lines::{max, min};

#[derive(Default, Debug, Clone)]
pub(crate) struct Series(pub Vec<(f64, f64)>);

impl From<&[f64]> for Series {
    fn from(vals: &[f64]) -> Self {
        Self::with_offset(vals, 0.0)
    }
}

impl AsRef<Vec<(f64, f64)>> for Series {
    fn as_ref(&self) -> &Vec<(f64, f64)> {
        &self.0
    }
}

impl IntoIterator for Series {
    type Item = (f64, f64);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Series {
    #[inline]
    pub(crate) fn with_offset(vals: &[f64], x_offset: f64) -> Self {
        Self(
            vals.iter()
                .enumerate()
                .map(|(i, v)| {
                    debug_assert!(!v.is_nan());
                    ((i as f64) + x_offset, *v)
                })
                .collect(),
        )
    }

    /// Get the x range of the series
    pub(crate) fn x_range(&self) -> std::ops::Range<f64> {
        if self.0.is_empty() {
            return f64::MIN..f64::MAX;
        }
        let min = self.0.iter().fold(self.0[0].0, |m, v| min(m, v.0));
        let max = self.0.iter().fold(self.0[0].0, |m, v| max(m, v.0));
        if min == max {
            return min..max + 1.0;
        }
        min..max
    }

    /// Get the y range of the series
    pub(crate) fn y_range(&self) -> std::ops::Range<f64> {
        if self.0.is_empty() {
            return f64::MIN..f64::MAX;
        }
        let min = self.0.iter().fold(self.0[0].1, |m, v| min(m, v.1));
        let max = self.0.iter().fold(self.0[0].1, |m, v| max(m, v.1));
        if min == max {
            return min..max + 1.0;
        }
        min..max
    }

    #[inline(always)]
    pub(crate) fn inner(self) -> Vec<(f64, f64)> {
        self.0
    }
}
