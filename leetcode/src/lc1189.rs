use std::cmp::min;
use std::collections::HashMap;

fn max_number_of_balloons(text: String) -> i32 {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in text.chars() {
        let current = map.get(&c).unwrap_or(&0) + 1;
        map.insert(c, current);
    }
    let mut result = i32::MAX;
    for c in "ban".chars() {
        result = min(*map.get(&c).unwrap_or(&0), result)
    }
    for c in "lo".chars() {
        result = min(*map.get(&c).unwrap_or(&0) / 2, result)
    }

    result
}

#[test]
fn test_input() {
    assert_eq!(1, max_number_of_balloons(String::from("nlaebolko")));
    assert_eq!(2, max_number_of_balloons(String::from("loonbalxballpoon")));
    assert_eq!(0, max_number_of_balloons(String::from("leetcode")));
}
