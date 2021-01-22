//! Bucketize is a crate for slotting values in a bucket
//! To do this create a Bucketizer and add yout buckets to it,
//! then use the '.bucketize()' to get back the bucket a value fits into,
//! 
//! # Example
//! ```
//! use bucketize::Bucketizer;
//!  
//! let b = Bucketizer::new()
//!     .bucket(Some(-1.0), Some(0.0), -0.5)
//!     .bucket(Some(0.0), Some(1.0), 0.5);
//!     assert_eq!(b.bucketize(99.97), None);
//! ```
//! 
//! Bucketizer holds the list of bucket you want to slot values into,
//! and does the 'bucketization' operation
//! 
//! You can create one with 'new()' and add with chained 'bucket()' calls.
//! Buckets are min inclusive and max esclusive, if a given value matches no bucket,
//! Bucktize returns None.

#[derive(Clone, Debug, PartialEq)]
pub struct Bucketizer {
    buckets: Vec<Bucket>,

}

type Bucket = (Option<f64>, Option<f64>, f64);

impl Bucketizer {
    pub fn new() -> Self {
        Bucketizer {
            buckets: Vec::new()
        }
    }

    pub fn bucket (self, min: Option<f64>, max: Option<f64>, value: f64) -> Self {
        let mut new = self;

        new.buckets.push((min, max, value));

        return new;
    }

    pub fn bucketize (&self, input: f64) -> Option<f64> {
        for bucket in &self.buckets {
            match *bucket {
                (None, None, val) => return Some(val),
                (Some(min), None, val) => {
                    if input >= min { return Some(val) }
                },
                (None, Some(max), val) => {
                    if input < max { return Some(val)}
                },
                (Some(min), Some(max), val) => {
                    if input >= min && input < max { return Some(val) }
                }
            }
        }

        return None;
    }
}



#[cfg(test)]
mod tests {
    use super::Bucketizer;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn single_bucket_middle_values () {
        let bucketizer = Bucketizer::new()
            .bucket(Some(0.1), Some(1.0), 0.5);

        assert_eq!(bucketizer.bucketize(0.1), Some(0.5));
        assert_eq!(bucketizer.bucketize(99.6), None);
    }

    #[test]
    fn single_bucket_end_values () {
        let bucketizer = Bucketizer::new()
            .bucket(Some(0.1), Some(1.0), 0.5);

        assert_eq!(bucketizer.bucketize(0.1), Some(0.5));
        assert_eq!(bucketizer.bucketize(1.0), None);
    }

    #[test]
    fn multiples_buckets_closed_ends () {
        let bucketizer = Bucketizer::new()
            .bucket(Some(-1.0), Some(0.0), -0.5)
            .bucket(Some(0.0), Some(1.0), 0.5);


            assert_eq!(bucketizer.bucketize(0.0), Some(0.5));
            assert_eq!(bucketizer.bucketize(-0.7), Some(-0.5));
            assert_eq!(bucketizer.bucketize(99.97), None);
    }

    #[test]
    fn multiples_buckets_open_ends () {
        let bucketizer = Bucketizer::new()
            .bucket(Some(0.0), Some(1.0), 0.5)
            .bucket(Some(1.0), None, 1.5);


            assert_eq!(bucketizer.bucketize(0.0), Some(0.5));
            assert_eq!(bucketizer.bucketize(-0.7), None);
            assert_eq!(bucketizer.bucketize(99.97), Some(1.5));
    }
}