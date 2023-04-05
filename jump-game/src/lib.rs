pub struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        let mut memo = vec![0; nums.len()];
        memo[0] = nums[0];

        for (index, &num) in nums.iter().enumerate().skip(1) {
            memo[index] = std::cmp::max(num, memo[index - 1] - 1);
        }

        !memo.iter().take(memo.len() - 1).any(|e| *e == 0)
    }
}
