/* LeetCode Problem Question 1095: Find in Mountain Array

(This problem is an interactive problem.)

You may recall that an array arr is a mountain array if and only if:
    arr.length >= 3

There exists some i with 0 < i < arr.length - 1 such that:
    arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
    arr[i] > arr[i + 1] > ... > arr[arr.length - 1]

Given a mountain array mountainArr, return the minimum index such that mountainArr.get(index) == target. If such an index does not exist, return -1.

You cannot access the mountain array directly. You may only access the array using a MountainArray interface:
    MountainArray.get(k) returns the element of the array at index k (0-indexed).
    MountainArray.length() returns the length of the array.

Submissions making more than 100 calls to MountainArray.get will be judged Wrong Answer. Also, any solutions that attempt to circumvent the judge will result in disqualification.

https://leetcode.com/problems/find-in-mountain-array
*/

pub struct MountainArray {
    values: Vec<i32>,
}

#[allow(dead_code)]
impl MountainArray {
    fn new(values: Vec<i32>) -> Self {
        MountainArray { values }
    }

    fn get(&self, index: i32) -> i32 {
        self.values[index as usize]
    }

    fn length(&self) -> i32 {
        self.values.len() as i32
    }

    fn debug(&self) {
        println!("{:?}", self.values);
    }
}

pub struct Solution;

impl Solution {
    fn find_mountain_peak(mountain_arr: &MountainArray) -> (i32, i32) {
        if mountain_arr.length() == 1 {
            return (0, mountain_arr.get(0));
        }

        let mut start: i32 = 0;
        let mut end: i32 = mountain_arr.length() - 1;
        mountain_arr.debug();

        while start != end - 1 {
            let midpoint = (start + end) / 2;
            if mountain_arr.get(midpoint) > mountain_arr.get(midpoint - 1) {
                start = midpoint;
            } else {
                end = midpoint;
            }
            println!("start: {start:?}, end: {end:?}, midpoint: {midpoint:?}");
        }

        if mountain_arr.get(start) > mountain_arr.get(end) {
            (start, mountain_arr.get(start))
        } else {
            (end, mountain_arr.get(end))
        }
    }

    fn binary_search(
        mountain_arr: &MountainArray,
        value: i32,
        start: i32,
        end: i32,
    ) -> Option<usize> {
        let mut start = start;
        let mut end = end;

        while start <= end {
            let midpoint = (start + end) / 2;
            let midpoint_value = mountain_arr.get(midpoint);
            match midpoint_value.cmp(&value) {
                std::cmp::Ordering::Equal => return Some(midpoint as usize),
                std::cmp::Ordering::Less => start = midpoint + 1,
                std::cmp::Ordering::Greater => end = midpoint - 1,
            }
        }

        None
    }

    pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        let (peak_index, _) = Solution::find_mountain_peak(mountain_arr);

        match Self::binary_search(mountain_arr, target, 0, peak_index) {
            Some(v) => v as i32,
            None => {
                match Self::binary_search(
                    mountain_arr,
                    target,
                    peak_index,
                    mountain_arr.length() - 1,
                ) {
                    Some(v) => v as i32,
                    None => -1,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MountainArray, Solution};

    #[test]
    fn test_find_mountain_peak_1() {
        let mountain = MountainArray::new(vec![1, 2, 3, 2, 1]);
        assert_eq!(Solution::find_mountain_peak(&mountain), (2, 3));
    }

    #[test]
    fn test_find_mountain_peak_2() {
        let mountain = MountainArray::new(vec![1, 2, 3, 4, 2, 1]);
        assert_eq!(Solution::find_mountain_peak(&mountain), (3, 4));
    }

    #[test]
    fn test_find_mountain_peak_3() {
        let mountain = MountainArray::new(vec![1]);
        assert_eq!(Solution::find_mountain_peak(&mountain), (0, 1));
    }

    #[test]
    fn test_find_mountain_peak_4() {
        let mountain = MountainArray::new(vec![1, 2, 3, 4, 5, 0]);
        assert_eq!(Solution::find_mountain_peak(&mountain), (4, 5));
    }

    #[test]
    fn test_find_mountain_peak_5() {
        let mountain = MountainArray::new(vec![1, 2, 3, 2, 1, 0]);
        assert_eq!(Solution::find_mountain_peak(&mountain), (2, 3));
    }

    #[test]
    fn test_find_mountain_peak_6() {
        let mountain = MountainArray::new(vec![1, 2, 3, 4, 5, 4, 3, 2, 1]);
        assert_eq!(Solution::find_mountain_peak(&mountain), (4, 5));
    }

    #[test]
    fn test_find_mountain_peak_7() {
        let mountain = MountainArray::new(vec![1, 2, 3, 4, 5, 4, 3, 2]);
        assert_eq!(Solution::find_mountain_peak(&mountain), (4, 5));
    }
}
