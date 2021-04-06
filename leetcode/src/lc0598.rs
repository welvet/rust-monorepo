use std::cmp::min;

pub fn max_count(_m: i32, _n: i32, ops: Vec<Vec<i32>>) -> i32 {
    let mut m_min = i32::MAX;
    let mut n_min = i32::MAX;

    for op in ops {
        m_min = min(m_min, op[0]);
        n_min = min(n_min, op[1]);
    }

    return n_min * m_min;
}

#[test]
fn test_case1() {
    assert_eq!(4, max_count(3, 3, vec!(vec!(2, 2), vec!(3, 3))));
}
