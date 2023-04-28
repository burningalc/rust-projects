use similar_string_groups::Solution;

#[test]
fn test_is_similar_string() {
    let input_1 = String::from("hello");
    let input_2 = String::from("hello");

    let input_3 = String::from("world1");
    let input_4 = String::from("world2");

    assert!(Solution::is_similar(&input_1, &input_2));
    assert!(!Solution::is_similar(&input_3, &input_4));
}

#[test]
fn test_1() {
    let input = vec!["tars", "rats", "arts", "star"];
    let input = input.iter().map(|f| f.to_string()).collect();
    let expected_output = 2;

    assert_eq!(Solution::num_similar_groups(input), expected_output);
}

#[test]
fn test_2() {
    let input = vec!["omv", "ovm"];
    let input = input.iter().map(|f| f.to_string()).collect();
    let expected_output = 1;

    assert_eq!(Solution::num_similar_groups(input), expected_output);
}

#[test]
fn test_3() {
    let input = vec![
        "kccomwcgcs",
        "socgcmcwkc",
        "sgckwcmcoc",
        "coswcmcgkc",
        "cowkccmsgc",
        "cosgmccwkc",
        "sgmkwcccoc",
        "coswmccgkc",
        "kowcccmsgc",
        "kgcomwcccs",
    ];
    let input = input.iter().map(|f| f.to_string()).collect();
    let expected_output = 5;

    assert_eq!(Solution::num_similar_groups(input), expected_output);
}
