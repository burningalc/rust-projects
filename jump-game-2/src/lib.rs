pub struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // sliding window
        let mut right: usize = 0;
        let mut left: usize = 0;

        let mut jump: i32 = 0;

        let mut nums_iter = nums.iter();

        while left < nums.len() - 1 {
            // jump to the furthest among all steps inside the window
            let mut furthest = 0;
            for i in right..left + 1 {
                furthest = std::cmp::max(
                    furthest,
                    *nums_iter.next().unwrap_or(&0) - (left - i) as i32,
                );
            }
            // perform the jump
            right = left + 1;
            left += furthest as usize;
            jump += 1;
        }

        jump
    }
}
