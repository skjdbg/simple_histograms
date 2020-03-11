use crate::histogram::Histogram;

/// used to create a `Histogram`  
/// `values`       : the data to plot on the histogram  
/// `range`        : the range of data considered /!\ data outside the range are not considered  
/// `step_number`  : the number of "rectangles" to plot (I think it's also the number of "bucket" in other libraries)
pub struct HistogramBuilder {
    values: Vec<f64>,
    range: Ranges,
    step_number: u32,
}

/// Ranges are the intervals of values that will be considered when calling `as_hist`  
/// ie: any value outside of the interval will *not* be considered when calling `as_hist`
///
/// `Specified(min, max)` explicitly mentions the bounds.  
///  min must be smaller than max ! (a null interval is invalid)
///
/// `Unspecified` will go from the smallest to the largest value
pub enum Ranges {
    Unspecified,
    Specified(f64, f64),
}

impl HistogramBuilder {
    pub fn new(rng: Ranges, step_num: u32) -> Self {
        if let Ranges::Specified(min, max) = rng {
            assert!(
                min < max,
                "The Bounds are invalid. `Specified(min, max)` => min < max, actual {} < {}",
                min,
                max
            );
        }
        assert!(
            step_num > 0,
            "The number of steps should be > 0. actual: {}",
            step_num
        );
        HistogramBuilder {
            values: Vec::<f64>::new(),
            range: rng,
            step_number: step_num,
        }
    }

    pub fn from_vec(vec: Vec<f64>, rng: Ranges, step_num: u32) -> Self {
        if let Ranges::Specified(min, max) = rng {
            assert!(
                min < max,
                "The Bounds are invalid. `Specified(min, max)` => min < max, actual {} < {}",
                min,
                max
            );
        }
        assert!(
            step_num > 0,
            "The number of steps should be > 0. actual: {}",
            step_num
        );
        HistogramBuilder {
            values: vec,
            range: rng,
            step_number: step_num,
        }
    }

    /// sorts the values vector
    pub(crate) fn sort(&mut self) {
        self.values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    /// adds a value to the future histogram
    pub fn push(&mut self, val: f64) {
        self.values.push(val);
    }

    /// creates a `Histogram`  according to the parameters from this `HistogramBuilder`
    pub fn as_hist(&mut self) -> Histogram {
        self.sort();

        let lower_bound: f64;
        let upper_bound: f64;
        match self.range {
            Ranges::Unspecified => {
                lower_bound = *(self
                    .values
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap());
                upper_bound = *(self
                    .values
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap());
            }

            Ranges::Specified(min, max) => {
                lower_bound = min;
                upper_bound = max;
            }
        }

        let step = (upper_bound - lower_bound) / self.step_number as f64;
        let mut new_ys = Vec::<u32>::with_capacity(self.step_number as usize);
        let mut new_xs = Vec::<f64>::with_capacity(self.step_number as usize);

        // creates each `bucket` then pushes them to the bucket vector
        for i in 0..self.step_number {
            new_ys.push(
                self.values
                    .iter()
                    .skip_while(|&x| *x < lower_bound + step * i as f64)
                    .take_while(|&x| *x < lower_bound + step * (i + 1) as f64)
                    .count() as u32,
            );
            new_xs.push(lower_bound + step / 2.0 + step * i as f64);
        }
        Histogram::new(new_xs, new_ys, step)
    }
}
