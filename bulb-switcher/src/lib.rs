pub struct Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        // finding out how many of bulb is a perfect square
        // i.e. having an odd number of factors
        let mut on_bulb = 0;
        for i in 1..n + 1 {
            if (i as f64).sqrt().floor().powi(2) == i as f64 {
                on_bulb += 1;
            }
        }
        on_bulb
    }

    pub fn bulb_switch_2(n: i32) -> i32 {
        // a faster approach with maths
        // the number of perfect square within 1..n = ⌊√(n)⌋
        (n as f64).sqrt() as i32
    }
}
