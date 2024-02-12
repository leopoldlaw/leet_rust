
use multiset::HashMultiSet;

/**
 * Given two string s and t, return true if t is an anagram of s, and false otherwise
 * 
 * An Anagram is a word or phrase formed by rearranging the letters 
 * of a different word or phrase,
 * typically using all the original letters exactly once.
 */
pub fn is_anagram(s: String, t: String) -> bool {
    //Could use a hash table with a count value instead of multiset
    let mut ms = HashMultiSet::<char>::new();
    for c in s.chars() {
        ms.insert(c);
    }
    for c in t.chars() {
        if !ms.remove(&c) {
            return false;
        }
    }
    ms.is_empty()
}
