use gnuplot::{Color, Figure, PlotOption};

/// a structure mean to be plotted
/// can only be created through a `HistogramBuilder`
#[derive(Debug)]
pub struct Histogram {
    // ys.len() must equal xs.len() which
    // should be guaranteed by constructing with `HistogramBuilder`
    ys: Vec<u32>,
    xs: Vec<f64>,

    // the width of the columns to plot
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
        self.plot_with_option(&[Color("red")]);
    }

    pub fn plot_with_option(&self, option: &[PlotOption<&str>]) {

        let mut fg = Figure::new();
        let mut axes = fg.axes2d();

        for (i, y) in self.ys.iter().enumerate() {
            axes = axes.fill_between(
                vec![self.xs[i] - self.step / 2.0, self.xs[i] + self.step / 2.0],
                vec![0, 0],
                vec![y, y],
                option,
            );
        }

        fg.show().unwrap();
    }

    pub fn get_xs(&self) -> &Vec<f64> {
        &self.xs
    }

    pub fn get_ys(&self) -> &Vec<u32> {
        &self.ys
    }

    pub fn get_step(&self) -> f64 {
        self.step
    }
}
