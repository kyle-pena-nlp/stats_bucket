

use crate::stats_bucket::StatsBucket;
use crate::fixed_point_stuff::{into_i64, into_i64_array, into_fixed_point, into_fixed_point_array};
//use crate::stats_bucket::fixed_point_stuff::{into_I32F32, into_I32F32_array, into_I64, into_I64_array};
use shank::ShankAccount;
use borsh::{BorshDeserialize, BorshSerialize};

// I am representing the I32F32 fixed-points as I64's on-chain since they are bit-aligned in representation
#[derive(Clone, BorshSerialize, BorshDeserialize, ShankAccount)]
pub struct StatsBucketAccount {

    // calculated quantities
    pub variance : i64, // 8
    pub corrected_variance : i64, // 8
    pub stdev : i64, // 8
    pub corrected_stdev : i64, // 8
    pub skewness : i64, // 8
    pub excess_kurtosis : i64, // 8
    pub n : u32, // 4

    // essential information
    pub mean : i64, // 8
    pub minimum : i64, // 8
    pub maximum : i64, // 8
    pub moments : [i64;10], // 10 * 8
}

impl StatsBucketAccount {
    fn init(&mut self, stats_bucket : StatsBucket) {
        self.copy_from_stats_bucket(stats_bucket);
    }
    pub fn copy_from_stats_bucket(&mut self, stats_bucket : StatsBucket) {
        // core info
        self.mean = into_i64(stats_bucket.sample_mean());
        self.minimum = into_i64(stats_bucket.minimum());
        self.maximum = into_i64(stats_bucket.maximum());
        self.moments = into_i64_array(&stats_bucket.moments());
        // calculated info
        self.variance = into_i64(stats_bucket.sample_variance());
        self.corrected_variance = into_i64(stats_bucket.corrected_sample_variance());
        self.stdev = into_i64(stats_bucket.sample_stdev());
        self.corrected_stdev = into_i64(stats_bucket.corrected_sample_stdev());
        self.skewness = into_i64(stats_bucket.sample_skewness());
        self.excess_kurtosis = into_i64(stats_bucket.sample_excess_kurtosis());
        self.n = into_i64(stats_bucket.n()) as u32;
    }
    pub fn as_stats_bucket(&self) -> StatsBucket {
        StatsBucket::init_from_stats(into_fixed_point(self.mean), into_fixed_point(self.minimum), into_fixed_point(self.maximum), &into_fixed_point_array(&self.moments))
    }
    pub fn get_size() -> usize {
        return 10 * 8 + (10 * 8);
    }

}
