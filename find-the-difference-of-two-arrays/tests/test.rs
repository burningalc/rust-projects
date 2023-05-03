use find_the_difference_of_two_arrays::Solution;
use std::collections::HashSet;
#[test]
fn test_1() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![2, 4, 6];
    let expected_output = vec![HashSet::from([1, 3]), HashSet::from([4, 6])];

    assert_eq!(
        Solution::find_difference(nums1, nums2)
            .iter()
            .map(|f| f.iter().cloned().collect::<HashSet<i32>>())
            .collect::<Vec<HashSet<i32>>>(),
        expected_output
    );
}
