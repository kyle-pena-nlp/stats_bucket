use num_integer::binomial;
use fixed::types::I32F32;
use crate::fixed_point_stuff::{FixedPowI, ONE, THREE, ZERO};

pub struct StatsBucket {
    mean : I32F32,
    minimum : I32F32,
    maximum : I32F32,
    moments : [I32F32;10]
}

impl StatsBucket {
    pub fn init_empty() -> Self {
        StatsBucket { 
            mean : ZERO,
            minimum : ZERO,
            maximum : ZERO, 
            moments : [ZERO;10] 
        }
    }
    
    pub fn init_from_stats(mean : I32F32, minimum : I32F32, maximum : I32F32, moments : &[I32F32;10]) -> Self {
        StatsBucket {
            mean,
            minimum,
            maximum,
            moments: moments.clone()
        }
    }

    pub fn n(&self) -> I32F32 {
        self.moments[0]
    }

    pub fn sample_mean(&self) -> I32F32 {
        self.mean
    }

    pub fn minimum(&self) -> I32F32 {
        return self.minimum;
    }

    pub fn maximum(&self) -> I32F32 {
        return self.maximum;
    }

    pub fn moments(&self) -> &[I32F32;10] {
        return &self.moments;
    }    

    pub fn sample_variance(&self) -> I32F32 {
        self.moments[2] / self.n()
    }

    pub fn corrected_sample_variance(&self) -> I32F32 {
        self.moments[2] / (self.n() - ONE)
    }

    pub fn sample_stdev(&self) -> I32F32 {
        self.sample_variance().sqrt()
    }

    pub fn corrected_sample_stdev(&self) -> I32F32 {
        self.corrected_sample_variance().sqrt()
    }

    pub fn sample_skewness(&self) -> I32F32 {
        let denom = self.moments[2]/self.n();
        let denom_exp_1_5 = (denom * denom * denom).sqrt();
        return (self.moments[3] / self.n()) / denom_exp_1_5;
    }

    pub fn sample_excess_kurtosis(&self) -> I32F32 {
        return (self.moments[4]/self.n()) / (self.moments[2]/self.n()).powi_positive(2) - THREE
    }

    pub fn initialize(&mut self, ys : &[I32F32]) {
        self.update(ys);
    }

    pub fn update<'a, I>(&mut self, ys: I)
    where
        I: IntoIterator<Item = &'a I32F32>,
    {
        let mut i = self.moments[0].to_num::<i32>();
        for &y in ys {
            if i == 0 {
                self.moments = {
                    let mut arr = [ZERO; 10];
                    arr[0] = ONE;
                    arr
                };
                self.mean = y;
                self.minimum = y;
                self.maximum = y;
            } else {
                let (new_mean, new_minimum, new_maximum) =
                    StatsBucket::update_stats(&mut self.moments, self.mean, self.minimum, self.maximum, y);
                self.mean = new_mean;
                self.minimum = new_minimum;
                self.maximum = new_maximum;
            }
            i += 1;
        }
    }

    pub fn combine(&mut self, other : &StatsBucket) {
        let (M1s,M2s,u1,u2) = (&self.moments, &other.moments, &self.mean, &other.mean);
        let means = (self.mean, other.mean);
        let mins = (self.minimum, other.minimum);
        let maxes = (self.maximum, other.maximum);
        let (new_moments, new_mean, new_min, new_max) = StatsBucket::combine_stats(M1s, M2s, means, mins, maxes);
        self.moments.copy_from_slice(&new_moments);
        self.mean = new_mean;
        self.minimum = new_min;
        self.maximum = new_max;
    }

    fn update_stats(moments : &mut [I32F32;10], mean : I32F32, minimum : I32F32, maximum : I32F32, y : I32F32) -> (I32F32,I32F32,I32F32) {
        for p in (0..=10).rev() {
            moments[p] = StatsBucket::calculate_updated_moment(p,moments,mean,y)
        }

        let new_n = moments[0];
        let new_mean = mean + (y-mean) / new_n;
        let new_min = I32F32::min(minimum,y);
        let new_max = I32F32::max(maximum,y);

        return (new_mean, new_min, new_max);
    }

    fn calculate_updated_moment(p : usize, moments: &[I32F32], mean : I32F32, y : I32F32) -> I32F32 {
        let s21 = y - mean;
        let n = moments[0] + ONE;
        let n1 = moments[0];
        let n2 = ONE;
        let mut Σ = ZERO;
        for k in 0..=p {
            Σ += I32F32::from_num(binomial(p,k) as i64) * ( moments[p-k] * (s21*(-n2/n)).powi_positive(k as u32) );
        }
        Σ += (s21*n1/n).powi_positive(p as u32);
        return Σ; 
    }

    fn combine_stats(M1s : &[I32F32;10], M2s : &[I32F32;10], means : (I32F32,I32F32), mins : (I32F32,I32F32), maxes : (I32F32,I32F32)) -> ([I32F32;10], I32F32, I32F32, I32F32) {
        let (u1,u2) = means;
        let (min1,min2) = mins;
        let (max1,max2) = maxes;
        let s21 = u2 - u1;
        let mut moments : [I32F32;10] = [ZERO;10];
        for p in (0..=10).rev() {
            moments[p] = StatsBucket::calculate_combined_moment(p, M1s, M2s, s21)
        }
        let n1 = M1s[0];
        let n2 = M2s[0];
        let new_mean = StatsBucket::calculate_combined_mean((u1,u2),(n1,n2));
        let new_min = I32F32::min(min1,min2);
        let new_max = I32F32::max(max1,max2);
        return (moments, new_mean, new_min, new_max);
    }

    fn calculate_combined_moment(p : usize, M1s : &[I32F32;10], M2s: &[I32F32;10], s21 : I32F32) -> I32F32 {
        let (n,n1,n2) = (M1s[0] + M2s[0], M1s[0], M2s[0]);
        let mut Σ = ZERO;
        for k in 0..=p {
            Σ += I32F32::from_num(binomial(p,k)) * ((M1s[p-k] * (s21*(-n2/n)).powi_positive(k as u32) ) + (M2s[p-k] * (s21*n1/n).powi_positive(k as u32)) )
        }
        return Σ;
    }

    fn calculate_combined_mean(means : (I32F32,I32F32), ns : (I32F32,I32F32)) -> I32F32 {
        let (u1,u2) = means;
        let (n1,n2) = ns;
        let n = n1 + n2;
        return u1 * (n1/n) + u2 * (n2/n);
    }
}