use std::cmp::max;

fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
    let mut k_vec: Vec<i32> = Vec::new();
    while k != 0 {
        k_vec.push(k % 10);
        k = k / 10;
    }

    num.reverse();

    let mut res: Vec<i32> = Vec::new();
    let mut next = 0;
    for i in 0..max(num.len(), k_vec.len()) {
        let mut curr = next;
        if i < num.len() {
            curr += num[i];
        }
        if i < k_vec.len() {
            curr += k_vec[i];
        }
        res.push(curr % 10);
        next = curr / 10;
    }

    if next != 0 {
        res.push(next);
    }
    res.reverse();

    return res;
}

#[test]
fn test_cases() {
    assert_eq!(vec!(1, 2, 3, 4), add_to_array_form(vec!(1, 2, 0, 0), 34));
}
