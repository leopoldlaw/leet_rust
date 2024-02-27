use std::usize;

/**
* There is a long table with a line of plates and candles arranged on top of it. You are given a 0-indexed string s consisting of characters '*' and '|' only, where a '*' represents a plate and a '|' represents a candle.
*
* You are also given a 0-indexed 2D integer array queries where queries[i] = [lefti, righti] denotes the substring s[lefti...righti] (inclusive).
* For each query, you need to find the number of plates between candles that are in the substring.
* A plate is considered between candles if there is at least one candle to its left and at least one candle to its right in the substring.
*
* For example, s = "||**||**|*", and a query [3, 8] denotes the substring "*||**|". The number of plates between candles in this substring is 2, as each of the two plates has at least one candle in the substring to its left and right.
* Return an integer array answer where answer[i] is the answer to the ith query.
*/
pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut prefix_sum = vec![0; s.len()];
    let mut inside = 0;
    let mut numplates = 0;
    let mut i = 0;
    prefix_sum[0] = 0;
    for c in s.chars() {
        if c == '|' {
            if inside == 0 {
                inside = 1;
            } else {
                prefix_sum[i] = numplates;
                numplates = 0;
            }
        } else if c == '*' && inside == 1 {
            numplates += 1;
        }
        if i > 0 {
            prefix_sum[i] += prefix_sum[i - 1];
        }
        i += 1;
    }
    let mut result = vec![0; queries.len()];
    i = 0;
    for query in queries {
        let start: usize = query[0].try_into().unwrap();
        let end: usize = query[1].try_into().unwrap();
        result[i] = prefix_sum[end] - prefix_sum[start];
        i += 1;
    }
    result
}
