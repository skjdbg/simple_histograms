use gnuplot::{Color, Figure, PlotOption};

/// `values`       : the data to plot on the histogram  
/// `range`        : the range of data considered /!\ data outside the range are not considered  
/// `step_number`  : the number of "rectangles" to plot (I think it's also the number of "bucket" in other libraries)
#[derive(Debug)]
pub struct Histogram {
    // ys.len() must equal xs.len()
    ys: Vec<u32>,
    xs: Vec<f64>,

    // the width of the columns to plot
    step: f64,
}

impl Histogram {
    fn new(new_xs: Vec<f64>, new_ys: Vec<u32>, new_step: f64) -> Histogram {
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

pub fn from_vec(values_slice: &[f64], rng: Range, step_num: u32) -> Histogram {
    assert!(
        step_num > 0,
        "The number of steps should be > 0. actual: {}",
        step_num
    );
    let mut values = values_slice.to_vec();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let lower_bound: f64;
    let upper_bound: f64;
    match rng {
        Range::Unspecified => {
            lower_bound = *(values
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap());
            upper_bound = *(values
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap());
        }

        Range::Specified(min, max) => {
            assert!(
                min < max,
                "The Bounds are invalid. `Specified(min, max)` => min < max, actual {} < {}",
                min,
                max
            );
            lower_bound = min;
            upper_bound = max;
        }
    }

    let step = (upper_bound - lower_bound) / step_num as f64;
    let mut new_ys = Vec::<u32>::with_capacity(step_num as usize);
    let mut new_xs = Vec::<f64>::with_capacity(step_num as usize);

    // creates each `bucket` then pushes them to the bucket vector
    for i in 0..step_num {
        new_ys.push(
            values
                .iter()
                .skip_while(|&x| *x < lower_bound + step * i as f64)
                .take_while(|&x| *x <= lower_bound + step * (i + 1) as f64)
                .count() as u32,
        );
        new_xs.push(lower_bound + step / 2.0 + step * i as f64);
    }
    Histogram::new(new_xs, new_ys, step)
}

/// The Range is the interval of values from the vector that will be taken into consideration
/// when making the [`Histogram`]  
/// ie: any value outside of the interval will *not* be considered when calling `from_vec`
///
/// `Specified(min, max)` explicitly mentions the bounds.  
///  min must be smaller than max ! (a null interval is invalid)
///
/// `Unspecified` will go from the smallest to the largest value
#[derive(Debug)]
pub enum Range {
    Unspecified,
    Specified(f64, f64),
}
