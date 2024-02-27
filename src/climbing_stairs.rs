/**
* You are climbing a staircase. It takes n steps to reach the top
* Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
*/
pub fn climb_stairs(n: i32) -> i32 {
    let mut cache = vec![0; (n + 1) as usize];
    cache[1] = 1;
    if n > 1 {
        cache[2] = 2;
    }
    for i in 3..=n {
        cache[i as usize] = cache[(i - 1) as usize] + cache[(i - 2) as usize];
    }
    cache[n as usize]
}

pub fn climb_stairs_brute(n: i32) -> i32 {
    if n == 2 {
        return 2;
    }
    if n == 1 {
        return 1;
    }
    if n < 1 {
        return 0;
    }
    // If n = 3, then you say, how many ways to get to 1 (2 before), plus how many ways to get 1
    // before.
    climb_stairs(n - 1) + climb_stairs(n + 2)
}

#[test]
fn climb_stairs1() {
    assert_eq!(3, climb_stairs(3))
}
