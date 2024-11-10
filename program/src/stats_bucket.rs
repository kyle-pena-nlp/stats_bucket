use num_integer::binomial;

pub struct StatsBucket {
    mean : f32,
    minimum : f32,
    maximum : f32,
    moments : [f32;10]
}

impl StatsBucket {

    pub fn init_empty() -> Self {
        StatsBucket { 
            mean : 0f32, 
            minimum : 0f32, 
            maximum : 0f32, 
            moments : [0f32;10] 
        }
    }
    
    pub fn init_from_stats(mean : f32, minimum : f32, maximum : f32, moments : [f32;10]) -> Self {
        StatsBucket {
            mean,
            minimum,
            maximum,
            moments
        }
    }

    pub fn n(&self) -> f32 {
        self.moments[0]
    }

    pub fn sample_mean(&self) -> f32 {
        self.mean
    }

    pub fn sample_variance(&self) -> f32 {
        self.moments[2] / self.n()
    }

    pub fn corrected_sample_variance(&self) -> f32 {
        self.moments[2] / (self.n() - 1f32)
    }

    pub fn sample_stdev(&self) -> f32 {
        self.sample_variance().sqrt()
    }

    pub fn corrected_sample_stdev(&self) -> f32 {
        self.corrected_sample_variance().sqrt()
    }

    pub fn sample_skewness(&self) -> f32 {
        return (self.moments[3] / self.n()) / (self.moments[2]/self.n()).powf(1.5f32)
    }

    pub fn sample_excess_kurtosis(&self) -> f32 {
        return (self.moments[4]/self.n()) / (self.moments[2]/self.n()).powf(2f32) - 3f32
    }

    pub fn initialize(&mut self, ys : &[f32]) {
        let mut i = 0;
        for y in ys {
            if i == 0 {
                self.moments = {
                    let mut arr = [0.0f32; 10];
                    arr[0] = 1.0f32;
                    arr
                };
                self.mean = *y;
                self.minimum = *y;
                self.maximum = *y;
            }
            else {
                let (new_mean, new_minimum, new_maximum) = StatsBucket::update_stats(&mut self.moments, self.mean, self.minimum, self.maximum, *y);
                self.mean = new_mean;
                self.minimum = new_minimum;
                self.maximum = new_maximum;
            }
            i += 1;
        }
    }

    fn update_stats(moments : &mut [f32;10], mean : f32, minimum : f32, maximum : f32, y : f32) -> (f32,f32,f32) {
        for p in (0..=10).rev() {
            moments[p] = StatsBucket::calculate_updated_moment(p,moments,mean,y)
        }

        let new_n = moments[0];
        let new_mean = mean + (y-mean) / new_n;
        let new_min = f32::min(minimum,y);
        let new_max = f32::max(maximum,y);

        return (new_mean, new_min, new_max);
    }

    fn calculate_updated_moment(p : usize, moments: &[f32], mean : f32, y : f32) -> f32 {
        let s21 = y - mean;
        let n = moments[0] + 1f32;
        let n1 = moments[0];
        let n2 = 1f32;
        let mut Σ = 0f32;
        
        for k in 0..=p {
            Σ += (binomial(p,k) as f32) * ( moments[p-k] * (s21*(-n2/n)).powf(k as f32) );
        }
        Σ += (s21*n1/n).powf(p as f32);
        return Σ; 
    }

    fn combine_stats(M1s : &[f32;10], M2s : &[f32;10], means : (f32,f32), u2 : f32, mins : (f32,f32), maxes : (f32,f32)) -> ([f32;10], f32, f32, f32) {
        
        let (u1,u2) = means;
        let (min1,min2) = mins;
        let (max1,max2) = maxes;

        let s21 = u2 - u1;
        let mut moments : [f32;10] = [0f32;10];

        for p in (0..=10).rev() {
            moments[p] = StatsBucket::calculate_combined_moment(p, M1s, M2s, s21)
        }

        let n1 = M1s[0];
        let n2 = M2s[0];

        let new_mean = StatsBucket::calculate_combined_mean((u1,u2),(n1,n2));
        let new_min = f32::min(min1,min2);
        let new_max = f32::max(max1,max2);

        return (moments, new_mean, new_min, new_max);
    }

    fn calculate_combined_moment(p : usize, M1s : &[f32;10], M2s: &[f32;10], s21 : f32) -> f32 {
        let (n,n1,n2) = (M1s[0] + M2s[0], M1s[0], M2s[0]);
        let mut Σ = 0.0f32;
        for k in 0..=p {
            Σ += (binomial(p,k) as f32) * ((M1s[p-k] * (s21*(-n2/n)).powf(k as f32) ) + (M2s[p-k] * (s21*n1/n).powf(k as f32) ))
        }
        return Σ;
    }

    fn calculate_combined_mean(means : (f32,f32), ns : (f32,f32)) -> f32 {
        let (u1,u2) = means;
        let (n1,n2) = ns;
        let n = n1 + n2;
        return u1 * (n1/n) + u2 * (n2/n);
    }
}