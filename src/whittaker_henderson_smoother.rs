use nalgebra::{DMatrix, Dyn};

/// Construct a d-th order difference matrix based on the initial N x N identity matrix.
///
/// # Arguments:
/// `n`: The number of datapoints in the input series
/// `order`: The order of the smoothing
///
/// # Returns:
/// A matrix of size (N - d) x N
fn difference_matrix(n: usize, order: usize) -> DMatrix<f64> {
    let mut coeffs = vec![0.0; 2 * order + 1];
    coeffs[order] = 1.0;

    for _i in 0..order {
        let n = coeffs.len();
        coeffs = vec_diff(&coeffs[..n - 1], &coeffs[1..]);
    }

    // Construct it row-by-row, each row having n floats.
    let n_rows = n - order;
    let mut rows = Vec::with_capacity(n_rows);
    for i in 0..n_rows {
        let mut vals = vec![0.0; n];
        for j in 0..=order {
            vals[i + j] = coeffs[j];
        }
        rows.append(&mut vals);
    }

    DMatrix::from_row_slice_generic(Dyn(n_rows), Dyn(n), &rows)
}

/// Compute the element wise difference of two vectors
#[inline]
fn vec_diff(a: &[f64], b: &[f64]) -> Vec<f64> {
    Vec::from_iter(a.iter().zip(b).map(|(a, b)| *a - *b))
}

/// Whittaker smoother (aka Whittaker-Henderson or Whittaker-Eilers depending on who you ask)
/// It is a discrete-time version of spline smoothing for equally spaced data.
///
/// # Arguments:
/// `vals`: The values to smooth
/// `lambda`: Controls the smoothing strength, the larger, the smoother
/// `order`: The order of the filter
///
/// # Returns:
/// If Ok, the smoothed `vals` series
pub fn whittaker_smoother(vals: &[f64], lambda: f64, order: usize) -> Option<Vec<f64>> {
    let n = vals.len();
    let ident = DMatrix::<f64>::identity(n, n);
    let d = difference_matrix(n, order);

    let a = d.transpose() * &d;
    let coefmat = ident + lambda * a;
    let val_mat = DMatrix::from_column_slice_generic(Dyn(n), Dyn(1), vals);
    let smoothed = coefmat.lu().solve(&val_mat)?;

    Some(Vec::from_iter(smoothed.iter().cloned()))
}

#[cfg(test)]
mod tests {
    use plotters::prelude::*;

    use crate::{lines::plot_lines, series::Series};

    use super::*;

    /// Data from this repo:
    /// <https://github.com/mhvwerts/whittaker-eilers-smoother>
    const WOOD_DATASET: &[u32] = &[
        106, 111, 111, 107, 105, 107, 110, 108, 111, 119, 117, 107, 105, 107, 109, 105, 104, 102,
        108, 113, 113, 107, 103, 103, 98, 102, 103, 104, 105, 105, 105, 101, 103, 107, 109, 104,
        100, 103, 100, 105, 102, 105, 106, 107, 104, 107, 109, 108, 111, 107, 107, 106, 107, 102,
        102, 101, 103, 103, 103, 100, 101, 101, 100, 102, 101, 96, 96, 98, 104, 107, 107, 102, 105,
        101, 105, 110, 111, 111, 100, 102, 102, 107, 112, 114, 113, 108, 106, 103, 103, 101, 103,
        106, 107, 106, 107, 107, 104, 111, 117, 118, 115, 107, 110, 117, 121, 122, 123, 119, 117,
        118, 115, 111, 108, 107, 105, 105, 105, 103, 105, 107, 109, 110, 111, 108, 107, 106, 108,
        107, 105, 102, 101, 102, 101, 97, 100, 105, 108, 108, 105, 103, 103, 100, 103, 106, 107,
        97, 98, 100, 101, 97, 99, 101, 104, 107, 109, 111, 109, 103, 105, 102, 108, 113, 113, 108,
        107, 102, 106, 106, 106, 103, 97, 103, 107, 102, 107, 111, 110, 107, 103, 99, 97, 99, 100,
        99, 100, 99, 100, 99, 99, 98, 100, 102, 102, 106, 112, 113, 109, 107, 105, 97, 105, 110,
        113, 108, 101, 95, 99, 100, 97, 92, 98, 101, 103, 101, 92, 95, 91, 86, 86, 87, 93, 97, 95,
        91, 86, 87, 88, 88, 89, 87, 90, 88, 87, 89, 90, 90, 87, 86, 88, 83, 85, 85, 87, 91, 93, 96,
        95, 89, 89, 85, 88, 89, 92, 95, 91, 87, 83, 83, 82, 81, 81, 80, 81, 82, 80, 76, 72, 73, 75,
        77, 75, 80, 81, 81, 81, 81, 81, 84, 86, 87, 88, 86, 84, 82, 80, 79, 82, 82, 76, 81, 83, 82,
        81, 75, 78, 78, 78, 79, 82, 82, 84, 82, 77, 77, 77, 75, 77, 73, 75, 76, 80, 77, 68, 71, 71,
        68, 67, 69, 72, 82,
    ];

    #[test]
    fn test_sparse_difference_matrix() {
        assert_eq!(
            difference_matrix(4, 3),
            DMatrix::from_row_slice_generic(Dyn(1), Dyn(4), &vec![-1.0, 3.0, -3.0, 1.0])
        );
        assert_eq!(
            difference_matrix(5, 3),
            DMatrix::from_row_slice_generic(
                Dyn(2),
                Dyn(5),
                &vec![-1.0, 3.0, -3.0, 1.0, 0.0, 0.0, -1.0, 3.0, -3.0, 1.0]
            )
        );
        assert_eq!(
            difference_matrix(6, 3),
            DMatrix::from_row_slice_generic(
                Dyn(3),
                Dyn(6),
                &vec![
                    -1.0, 3.0, -3.0, 1.0, 0.0, 0.0, 0.0, -1.0, 3.0, -3.0, 1.0, 0.0, 0.0, 0.0, -1.0,
                    3.0, -3.0, 1.0
                ]
            )
        );
    }

    #[test]
    fn plot_whittaker_smoother_on_wood() {
        let raw = Vec::from_iter(WOOD_DATASET.iter().map(|v| *v as f64));
        let lambda = 2e4;
        let order = 3;
        let smoothed = whittaker_smoother(&raw, lambda, order).unwrap();

        let filename = format!(
            "img/whittaker_on_wood_lambda_{}_order_{}.png",
            lambda, order
        );
        let root_area = BitMapBackend::new(&filename, (1024, 1024)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();
        let root_area = root_area
            .titled(&filename, ("sans-serif", 20).into_font())
            .unwrap();

        let raw_series = Series(Vec::from_iter(
            raw.iter().enumerate().map(|(i, v)| (i as f64, *v)),
        ));
        let smoothed_series = Series(Vec::from_iter(
            smoothed.iter().enumerate().map(|(i, v)| (i as f64, *v)),
        ));
        plot_lines(&root_area, &[(raw_series, BLACK), (smoothed_series, RED)]);
    }
}
