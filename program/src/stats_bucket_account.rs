use crate::stats_bucket::StatsBucket;
use shank::ShankAccount;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct StatsBucketAccount {

    // calculated quantities
    pub variance : f32, // 4
    pub corrected_variance : f32, // 4
    pub stdev : f32, // 4
    pub corrected_stdev : f32, // 4
    pub skewness : f32, // 4
    pub excess_kurtosis : f32, // 4
    pub n : u32, // 4

    // essential information
    pub mean : f32, // 4
    pub minimum : f32, // 4
    pub maximum : f32, // 4
    pub moments : [f32;10], // 10 * 4
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
        self.n = stats_bucket.n() as u32;
    }
    pub fn as_stats_bucket(&self) -> StatsBucket {
        StatsBucket::init_from_stats(self.mean, self.minimum, self.maximum, &self.moments)
    }
    pub fn get_size() -> usize {
        return 10 * 4 + (10 * 4);
    }
}