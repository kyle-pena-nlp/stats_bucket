use crate::stats_bucket::StatsBucket;
use shank::ShankAccount;
use borsh::{BorshDeserialize, BorshSerialize};
use std::convert::Into;

#[derive(Clone, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct StatsBucketAccount {

    // calculated quantities
    pub variance : f32,
    pub corrected_variance : f32,
    pub stdev : f32,
    pub corrected_stdev : f32,
    pub skewness : f32,
    pub excess_kurtosis : f32,
    pub n : usize,

    // essential information
    pub mean : f32,
    pub minimum : f32,
    pub maximum : f32,
    pub moments : [f32;10],
}

impl StatsBucketAccount {
    fn init(&mut self, stats_bucket : StatsBucket) {
        self.copy_from_stats_bucket(stats_bucket);
    }
    pub fn copy_from_stats_bucket(&mut self, stats_bucket : StatsBucket) {
        // core info
        self.mean = stats_bucket.sample_mean();
        self.minimum = stats_bucket.minimum();
        self.maximum = stats_bucket.maximum();
        self.moments.copy_from_slice(stats_bucket.moments());
        // calculated info
        self.variance = stats_bucket.sample_variance();
        self.corrected_variance = stats_bucket.corrected_sample_variance();
        self.stdev = stats_bucket.sample_stdev();
        self.corrected_stdev = stats_bucket.corrected_sample_stdev();
        self.skewness = stats_bucket.sample_skewness();
        self.excess_kurtosis = stats_bucket.sample_excess_kurtosis();
        self.n = stats_bucket.n() as usize;
    }
    pub fn as_stats_bucket(&self) -> StatsBucket {
        StatsBucket::init_from_stats(self.mean, self.minimum, self.maximum, &self.moments)
    }
}