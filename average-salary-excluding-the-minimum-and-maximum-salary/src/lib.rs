pub struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let max = *salary.iter().max().unwrap() as f64;
        let min = *salary.iter().min().unwrap() as f64;
        (salary.iter().sum::<i32>() as f64 - max - min) / (salary.len() - 2) as f64
    }

    pub fn average_single_pass(salary: Vec<i32>) -> f64 {
        let mut max = 999;
        let mut min = 10_000_000;
        let mut sum = 0;

        for s in &salary {
            sum += *s;

            if *s > max {
                max = *s;
            }

            if *s < min {
                min = *s;
            }
        }

        (sum - max - min) as f64 / (salary.len() - 2) as f64
    }
}
