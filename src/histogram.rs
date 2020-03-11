use gnuplot::{Color, Figure};

/// a structure mean to be plotted
/// can only be created through a `HistogramBuilder`
pub struct Histogram {
    // ys.len() must equal xs.len() which
    // should be guaranteed by constructing with HistogramBuilder
    ys: Vec<u32>,
    xs: Vec<f64>,

    //the width of the columns to plot
    step: f64,
}

impl Histogram {
    pub(crate) fn new(new_xs: Vec<f64>, new_ys: Vec<u32>, new_step: f64) -> Histogram {
        Histogram {
            ys: new_ys,
            xs: new_xs,
            step: new_step,
        }
    }

    pub fn plot(&self) {
        let mut fg = Figure::new();
        let mut axes = fg.axes2d();

        for (i, y) in self.ys.iter().enumerate() {
            axes = axes.fill_between(
                vec![self.xs[i] - self.step / 2.0, self.xs[i] + self.step / 2.0],
                vec![0, 0],
                vec![y, y],
                &[Color("red")],
            );
        }

        fg.show().unwrap();
    }
}
