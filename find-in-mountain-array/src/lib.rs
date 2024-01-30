struct MountainArray {
    values: Vec<i32>,
}

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

struct Solution;
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
    ) -> Option<(usize)> {
        return Some(0);
    }

    pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        let (peak_index, peak_value) = Solution::find_mountain_peak(mountain_arr);

        match binary_search() {
            Some(v) => v as i32,
            None => -1,
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
