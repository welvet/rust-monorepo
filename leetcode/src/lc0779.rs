fn kth_grammar(n: i32, k: i32) -> i32 {
    if n == 1 {
        return 0;
    }

    // 0->01 1->10

    let f = (k as f64) / 2.0;
    let prev = kth_grammar(n - 1, f.ceil() as i32);
    return if k % 2 != 0 {
        if prev == 0 {
            0
        } else {
            1
        }
    } else {
        if prev == 0 {
            1
        } else {
            0
        }
    };
}

#[test]
fn test_cases() {
    assert_eq!(1, kth_grammar(3, 3));
    assert_eq!(1, kth_grammar(2, 2));
    assert_eq!(0, kth_grammar(3, 1));
    assert_eq!(0, kth_grammar(1, 1));
    assert_eq!(0, kth_grammar(2, 1));
    assert_eq!(1, kth_grammar(2, 2));
    assert_eq!(1, kth_grammar(4, 5));
    assert_eq!(1, kth_grammar(30, 417219134));
}
