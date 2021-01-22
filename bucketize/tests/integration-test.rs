extern crate bucketize;
use bucketize::Bucketizer;

#[test]
fn multiples_buckets_open_ends () {
    let bucketizer = Bucketizer::new()
        .bucket(Some(0.0), Some(1.0), 0.5)
        .bucket(Some(1.0), None, 1.5);


        assert_eq!(bucketizer.bucketize(0.0), Some(0.5));
        assert_eq!(bucketizer.bucketize(-0.7), None);
        assert_eq!(bucketizer.bucketize(99.97), Some(1.5));
}