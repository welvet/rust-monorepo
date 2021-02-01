use std::collections::HashMap;
use std::convert::TryInto;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (pos, x) in nums.iter().enumerate() {
        if map.contains_key(&(target - x)) {
            result.push(*map.get(&(target - x)).unwrap());
            result.push(pos.try_into().unwrap());
        }
        map.insert(*x, pos.try_into().unwrap());
    }

    return result;
}

pub fn main() {
	let mut input: Vec<i32> = Vec::new();
	input.push(1);
	input.push(2);
	input.push(3);

	let result = two_sum(input, 4);

	for x in result {
		println!("{}", x);
	}
}

