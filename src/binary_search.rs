

/**
 * Given an array of integers nums which is sorted in ascending order,
 * and an integer target,
 * write a function to search target in nums.
 * If target exists, then return its index.
 * Otherwise, return -1.
 *
 * You must write an algorithm with O(log n) runtime complexity.
 */
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut high = nums.len()-1;
    let mut low = 0;

    while high > low {
        let midpnt = ((high - low) / 2) + low;
        print!("{}, : {}\n", midpnt, nums[midpnt]);
        match nums[midpnt] {
            x if x == target => return midpnt.try_into().unwrap(),
            x if x < target => low = midpnt+1,
            x if x > target => high = midpnt-1,
            _ => panic!("Impossible"),
        }
    }
    -1
}

