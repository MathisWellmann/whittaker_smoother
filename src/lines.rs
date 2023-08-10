use plotters::{coord::Shift, prelude::*};

use crate::series::Series;

pub(crate) fn min(v0: f64, v1: f64) -> f64 {
    if v0 < v1 {
        v0
    } else {
        v1
    }
}

pub(crate) fn max(v0: f64, v1: f64) -> f64 {
    if v0 > v1 {
        v0
    } else {
        v1
    }
}

pub(crate) fn plot_lines<'a>(
    subplot: &DrawingArea<BitMapBackend<'a>, Shift>,
    lines: &[(Series, RGBColor)],
) {
    let x_min = lines.iter().fold(lines[0].0.x_range().start, |acc, x| {
        min(acc, x.0.x_range().start)
    });
    let x_max = lines.iter().fold(lines[0].0.x_range().end, |acc, x| {
        max(acc, x.0.x_range().end)
    });
    let y_min = lines.iter().fold(lines[0].0.y_range().start, |acc, x| {
        min(acc, x.0.y_range().start)
    });
    let y_max = lines.iter().fold(lines[0].0.y_range().end, |acc, x| {
        max(acc, x.0.y_range().end)
    });

    subplot.fill(&WHITE).unwrap();
    let mut cc0 = ChartBuilder::on(subplot)
        .margin(5)
        .set_all_label_area_size(40)
        .caption("Prices", ("sans-serif", 15).into_font().with_color(BLACK))
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();

    cc0.configure_mesh()
        .x_labels(20)
        .y_labels(20)
        .x_label_formatter(&|v| format!("{v:.0}"))
        .y_label_formatter(&|v| format!("{v:.2}"))
        .draw()
        .unwrap();
    for (line, color) in lines.iter() {
        cc0.draw_series(LineSeries::new(line.clone().inner(), color).point_size(2))
            .unwrap();
    }
    cc0.configure_series_labels()
        .border_style(BLACK)
        .draw()
        .unwrap();
}
