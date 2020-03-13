# simple_histograms
## A very basic and simple of use Rust implementation of a histogram.

**This library is meant to plot histograms, not calculate quantiles, median, etc which other libraries already do.**

**Disclaimer**  
I am no expert. If you find another library that does the same, it probably does it better and faster. I don't know if I will ever update this project or add any functionalities.  
**No test have been run on this project**

I created this library for two reasons :
- I wanted to make histograms and plot them from vectors without having to write 20 lines of code and interacting with multiple libraries. I found some that created a histogram but none that allowed to directly plot them (or maybe i was too dumb to realize they did), or they required too much boilerplate.
- I'm new to rust and it's a good exercise. (expect errors)

The way this library works is pretty simple. 
1. Create a `Vec<f64>`, the put your data in it and process them in any way you like.
2. Turn it into a `HistogramBuilder` by specifying a `Range`, and a number of steps.
3. Make it into a `Histogram`.
4. Plot it, or use the Vectors generated.

Alternatively, you can directly create a `HistogramBuilder` and push values into it if you don't want to create an intermediary variable and don't need any processing to be done.

```Rust
use simple_histograms::{HistogramBuilder, Range};

let values = vec![1.0, 2.0, 3.0, 4.0, 4.0, 5.0];

let builder = HistogramBuilder::from_vec(values, Range::Specified(2.0, 4.0), 3);
let hist = builder.as_hist();

assert_eq!(hist.get_ys(), [1 1 2]);
```
