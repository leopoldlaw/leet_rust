use std::cmp;
use std::collections::HashMap;
/**
* Given a string s which consists of lowercase or uppercase letters, return the length
* of the longest palindrome that can be built with those letters.
*
* Letters are case sensitive, for example "Aa" is not considered a palindrome here.
*/
pub fn longest_palindrome(s: String) -> i32 {
    let mut hm: HashMap<char, i32> = std::collections::HashMap::new();
    for c in s.chars() {
        if hm.contains_key(&c) {
            hm.insert(c, hm.get(&c).unwrap() + 1);
        } else {
            hm.insert(c, 1);
        }
    }

    let mut len = s.len() as i32;
    let mut removed = false;

    for key in hm.keys() {
        if (hm.get(key).unwrap() % 2) != 0 {
            len -= 1;
            removed = true;
        }
    }

    if (removed) && ((len % 2) == 0) {
        len += 1;
    }
    len
}

pub fn longest_palindrome_substring(s: String) -> i32 {
    let mut i: usize = 0;
    let mut j: usize = s.len() - 1;
    let chars: Vec<char> = s.chars().collect();
    while i < j {
        if chars[i] != chars[j] {
            let left = s[(i + 1)..=j].to_string();
            let right = s[i..=(j - 1)].to_string();
            return cmp::max(longest_palindrome(left), longest_palindrome(right));
        }
        i += 1;
        j -= 1;
    }
    s.len() as i32
}
