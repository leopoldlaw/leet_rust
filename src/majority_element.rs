use core::num;
use std::collections::HashMap;
/**
* Given an array nums of size n, return the majority element
* The majority element is the element that appears more than n/2 times. You may assume that
* the majority element always exists in the array
* Moore's voting algorithm - The candidate will always win as there are a majority of them in the
* array, so you can just see who has the most occurances.
*/
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut candidate = 0;
    let mut count = 0;
    for num in nums {
        if count == 0 {
            candidate = num;
            count += 1;
        } else if num == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    candidate
}

pub fn majority_element_brute(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut hm: HashMap<i32, i32> = std::collections::HashMap::new();
    for num in nums {
        hm.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }
    for key in hm.keys() {
        let val = hm.get(key).unwrap().to_owned();
        if val > (len as i32) / 2 {
            return key.to_owned();
        }
    }
    -1
}
