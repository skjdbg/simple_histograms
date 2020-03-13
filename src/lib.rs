extern crate gnuplot;

pub mod histogram;

pub use histogram::{from_vec, Range};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::histogram::{from_vec, Range};

        let values = vec![1.0, 2.0, 3.0, 4.0, 4.0, 5.0];

        let hist = from_vec(&values, Range::Specified(2.0, 4.0), 3);

        assert_eq!(*hist.get_ys(), [1, 1, 2]);
    }
}
