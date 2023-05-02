pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => nums[0],
            2 => std::cmp::max(nums[0], nums[1]),
            _ => std::cmp::max(
                Self::rob_solver(nums[0..(nums.len() - 1)].to_vec()),
                Self::rob_solver(nums[1..].to_vec()),
            ),
        }
    }

    pub fn rob_solver(nums: Vec<i32>) -> i32 {
        let mut cache = vec![0; nums.len()];
        cache[0] = nums[0];
        cache[1] = std::cmp::max(nums[0], nums[1]);

        for (mut index, num) in nums.iter().skip(2).enumerate() {
            index += 2;
            cache[index] = std::cmp::max(num + cache[index - 2], cache[index - 1]);
        }

        *cache.last().unwrap()
    }
}
