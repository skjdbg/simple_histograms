extern crate gnuplot;

pub mod histogram;
pub mod histogram_builder;

pub use histogram_builder::Range;
pub use histogram_builder::HistogramBuilder;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
