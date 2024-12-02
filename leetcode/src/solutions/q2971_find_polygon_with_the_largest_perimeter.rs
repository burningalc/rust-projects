/* LeetCode Problem Question 2971: Find Polygon With the Largest Perimeter

You are given an array of positive integers nums of length n.

A polygon is a closed plane figure that has at least 3 sides. The longest side of a polygon is smaller than the sum of its other sides.

Conversely, if you have k (k >= 3) positive real numbers a1, a2, a3, ..., ak where a1 <= a2 <= a3 <= ... <= ak and a1 + a2 + a3 + ... + ak-1 > ak, then there always exists a polygon with k sides whose lengths are a1, a2, a3, ..., ak.

The perimeter of a polygon is the sum of lengths of its sides.

Return the largest possible perimeter of a polygon whose sides can be formed from nums, or -1 if it is not possible to create a polygon.

https://leetcode.com/problems/find-polygon-with-the-largest-perimeter/
*/

pub struct Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        if nums.len() < 3 {
            return -1;
        }

        nums.sort_unstable();

        let mut sum = nums[0] as i64;
        let mut largest_sum = (0, sum);

        for (index, &num) in nums.iter().enumerate().skip(1) {
            let num = num.into();

            if sum > num {
                largest_sum.0 = index;
                largest_sum.1 = sum + num;
            }
            sum += num;
        }

        if largest_sum.0 > 1 {
            largest_sum.1
        } else {
            -1
        }
    }
}

#[test]
fn test_1() {
    let actual = Solution::largest_perimeter(vec![5, 5, 5]);
    let expected = 15;
    assert_eq!(expected, actual);
}
#[test]
fn test_2() {
    let actual = Solution::largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]);
    let expected = 12;
    assert_eq!(expected, actual);
}

#[test]
fn test_3() {
    let actual = Solution::largest_perimeter(vec![5, 5, 50]);
    let expected = -1;
    assert_eq!(expected, actual);
}

#[test]
fn test_4() {
    let actual = Solution::largest_perimeter(vec![1, 2, 3]);
    let expected = -1;
    assert_eq!(expected, actual);
}

#[test]
fn test_5() {
    let actual = Solution::largest_perimeter(vec![1, 1, 2]);
    let expected = -1;
    assert_eq!(expected, actual);
}

#[test]
fn test_6() {
    let actual = Solution::largest_perimeter(vec![
        300005055, 352368231, 311935527, 315829776, 327065463, 388851949, 319541150, 397875604,
        311309167, 391897750, 366860048, 359976490, 325522439, 390648914, 359891976, 369105322,
        350430086, 398592583, 354559219, 372400239, 344759294, 379931363, 308829137, 335032174,
        336962933, 380797651, 378305476, 336617902, 393487098, 301391791, 394314232, 387440261,
        316040738, 388074503, 396614889, 331609633, 374723367, 380418460, 349845809, 318514711,
        308782485, 308291996, 375362898, 397542455, 397628325, 392446446, 368662132, 378781533,
        372327607, 378737987,
    ]);
    let expected = 17876942274;
    assert_eq!(expected, actual);
}
