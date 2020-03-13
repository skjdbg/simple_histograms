// use crate::histogram::Histogram;

// /// used to create a `Histogram`  
// /// 
// /// `values`       : the data to plot on the histogram  
// /// `range`        : the range of data considered /!\ data outside the range are not considered  
// /// `step_number`  : the number of "rectangles" to plot (I think it's also the number of "bucket" in other libraries)
// #[derive(Debug)]
// pub struct HistogramBuilder {
//     values: Vec<f64>,
//     range: Range,
//     step_number: u32,
// }


// impl HistogramBuilder {
//     pub fn new(rng: Range, step_num: u32) -> Self {
//         if let Range::Specified(min, max) = rng {
//             assert!(
//                 min < max,
//                 "The Bounds are invalid. `Specified(min, max)` => min < max, actual {} < {}",
//                 min,
//                 max
//             );
//         }
//         assert!(
//             step_num > 0,
//             "The number of steps should be > 0. actual: {}",
//             step_num
//         );
//         HistogramBuilder {
//             values: Vec::<f64>::new(),
//             range: rng,
//             step_number: step_num,
//         }
//     }

//     pub fn from_vec(vec: Vec<f64>, rng: Range, step_num: u32) -> Self {
//         if let Range::Specified(min, max) = rng {
//             assert!(
//                 min < max,
//                 "The Bounds are invalid. `Specified(min, max)` => min < max, actual {} < {}",
//                 min,
//                 max
//             );
//         }
//         assert!(
//             step_num > 0,
//             "The number of steps should be > 0. actual: {}",
//             step_num
//         );
//         HistogramBuilder {
//             values: vec,
//             range: rng,
//             step_number: step_num,
//         }
//     }


// }
