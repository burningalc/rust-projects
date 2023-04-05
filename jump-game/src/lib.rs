pub struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // create a memo table to record if an index can be reached
        let mut memo = vec![false; nums.len()];
        memo[0] = true;
        for (index, &num) in nums.iter().enumerate() {
            if index == nums.len() {
                break;
            }

            // only jump from this position if we can jump to this position
            if !memo[index] {
                continue;
            }

            // mark all possible jumping position to be true
            for i in 1..num + 1 {
                let jumping_index = index + i as usize;
                if jumping_index >= nums.len() {
                    break;
                }
                memo[jumping_index] = true;
            }
        }

        *memo.last().unwrap()
    }
}
