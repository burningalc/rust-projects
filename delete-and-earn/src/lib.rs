pub struct Solution;
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        // build a counter to count the occurrences of each num
        // get the largest num in nums
        let largest = nums.iter().max().unwrap();
        let mut occurrences: Vec<i32> = vec![0; *largest as usize];

        for &num in &nums {
            occurrences[num as usize - 1] += 1;
        }

        // transform to the score
        let scores: Vec<i32> = occurrences
            .iter()
            .enumerate()
            .map(|(index, &e)| e * (index as i32 + 1))
            .collect();

        // solve as a house robber problem
        if scores.len() == 1 {
            return scores[0];
        }

        let mut memo = vec![0; scores.len()];
        memo[0] = scores[0];
        memo[1] = std::cmp::max(scores[0], scores[1]);

        for (mut index, &score) in scores.iter().skip(2).enumerate() {
            index += 2;
            memo[index] = std::cmp::max(memo[index - 2] + score, memo[index - 1]);
        }

        *memo.last().unwrap()
    }
}
