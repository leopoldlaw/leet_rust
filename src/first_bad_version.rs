
/**
 * returns if version is bad
 */
fn is_bad_version(version: i32) -> bool {
    false
}

fn first_bad_version(n: i32) {
    let mut high = n;
    let mut low = 1;

    while(high > low) {
        let mid = ((high - low) / 2) + low;

        let val = verify_solution(mid);
        if val == 0 {
            return mid;
        }
        else if val < 0 {
            high = mid-1;
        }
        else {
            low = mid + 1;
        }
    }
    panic!("impossible");
}

fn verify_solution(n: i32) -> i32 {
    if is_bad_version(n) {
        if !is_bad_version(n-1) {
            return 0;
        }
        return -1;
    }
    1

}
