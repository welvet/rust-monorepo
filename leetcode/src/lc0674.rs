use std::cmp::max;

fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut max_res = 1;
    let mut result = 1;
    let mut prev = nums[0];

    for i in 1..nums.len() {
        if prev < nums[i] {
            result += 1;
        } else {
            max_res = max(result, max_res);
            result = 1;
        }

        prev = nums[i];
    }

    return max(result, max_res);
}

#[test]
fn test() {
    assert_eq!(3, find_length_of_lcis(vec!(1, 3, 5, 4, 7)));
    assert_eq!(1, find_length_of_lcis(vec!(2, 2)));
}
