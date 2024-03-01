use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
mod balanced_binary_tree;
mod binary_search;
mod climbing_stairs;
mod flood_fill;
mod invert_tree;
mod longest_palindrome;
mod lowest_common_ancestor;
mod majority_element;
mod plates_between_candles;
mod queue_with_stacks;
mod reverse_linked_list;
mod valid_anagram;
/**
* Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
* You may assume that each input would have exactly one solution, and you may not use the same element twice.
* You can return the answer in any order.
*/
pub fn twosum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = HashMap::new();
    for (k, &v) in nums.iter().enumerate() {
        let look = target - v;
        if let Some(&j) = hm.get(&look) {
            return vec![k as i32, j];
        }
        hm.insert(v, k as i32);
    }
    vec![]
}

/**
 * You are given an array prices where prices[i] is the price of a given stock on the ith day.
 * You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
 * Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.
 */
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut maxprofit = 0;
    let mut bought = prices[0];
    for (_i, &val) in prices.iter().enumerate() {
        if val < bought {
            bought = val
        }
        if maxprofit < (val - bought) {
            maxprofit = val - bought
        }
    }

    return maxprofit;
}
/**
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

 * An input string is valid if:

 * Open brackets must be closed by the same type of brackets.
 * Open brackets must be closed in the correct order.
 * Every close bracket has a corresponding open bracket of the same type.
*/
pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        if c == '(' {
            stack.push(')');
        } else if c == '[' {
            stack.push(']');
        } else if c == '{' {
            stack.push('}');
        } else {
            match stack.pop() {
                Some(val) => {
                    if val != c {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }
    stack.len() == 0
}

//Definition for singly-linked list. Provided by leetcode, non editable
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            left: None,
            right: None,
            val,
        }
    }
}

/**
 * You are given the heads of two sorted linked lists
 * Merge the two lists into one sorted list.
 * Return the head of the merged linked list
 */
pub fn mergeTwoLists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(l), None) => return Some(l),
        (None, Some(r)) => return Some(r),
        (None, None) => return None,
        (Some(l), Some(r)) => {
            if l.val <= r.val {
                return Some(Box::new(ListNode {
                    next: mergeTwoLists(l.next, Some(r)),
                    val: l.val,
                }));
            } else {
                return Some(Box::new(ListNode {
                    next: mergeTwoLists(Some(l), r.next),
                    val: r.val,
                }));
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn twosum1() {
        let result = twosum(vec![2, 7, 8, 3], 9);
        assert_eq!(result, vec![1, 0]);
    }
    #[test]
    fn twosum2() {
        let result = twosum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![2, 1]);
    }
    #[test]
    fn twosum3() {
        let result = twosum(vec![3, 3], 6);
        assert_eq!(result, vec![1, 0]);
    }
    #[test]
    fn profit1() {
        let result = max_profit(vec![7, 1, 5, 3, 6, 4]);
        assert_eq!(result, 5);
    }
    #[test]
    fn profit2() {
        let result = max_profit(vec![7, 6, 4, 3, 1]);
        assert_eq!(result, 0);
    }
    #[test]
    fn isvalid1() {
        let result = is_valid(String::from("()"));
        assert_eq!(result, true);
    }
    #[test]
    fn isvalid2() {
        let result = is_valid(String::from("()[]{}"));
        assert_eq!(result, true);
    }
    #[test]
    fn isvalid3() {
        let result = is_valid(String::from("(]"));
        assert_eq!(result, false);
    }
    #[test]
    fn mergeTwoLists1() {
        let left = Some(Box::new(ListNode::new(1)));
        let right = Some(Box::new(ListNode::new(2)));
        let result = mergeTwoLists(right, left);
        let first_node = ListNode {
            next: Some(Box::new(ListNode::new(2))),
            val: 1,
        };
        let expected = Some(Box::new(first_node));
        assert_eq!(result, expected);
    }
    #[test]
    fn mergeTwoLists2() {
        let left = None;
        let right = Some(Box::new(ListNode::new(2)));
        let expected = right.clone();
        let result = mergeTwoLists(right, left);
        assert_eq!(result, expected);
    }
    #[test]
    fn mergeTwoLists3() {
        let left = Some(Box::new(ListNode::new(2)));
        let right = None;
        let expected = left.clone();
        let result = mergeTwoLists(right, left);
        assert_eq!(result, expected);
    }
    #[test]
    fn mergeTwoLists4() {
        let left = None;
        let right = None;
        let result = mergeTwoLists(right, left);
        let expected = None;
        assert_eq!(result, expected);
    }
    #[test]
    fn invertTree1() {
        assert_eq!(invert_tree::invert_tree(None), None);
    }
    #[test]
    fn valid_anagram1() {
        assert_eq!(
            valid_anagram::is_anagram(String::from("rat"), String::from("cat")),
            false
        );
    }
    #[test]
    fn valid_anagram2() {
        assert_eq!(
            valid_anagram::is_anagram(String::from("anagram"), String::from("nagaram")),
            true
        );
    }
    #[test]
    fn binary_search1() {
        assert_eq!(binary_search::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }
    #[test]
    fn binary_search2() {
        assert_eq!(binary_search::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
    #[test]
    fn flood_fill1() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let solution = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(flood_fill::flood_fill(image, 1, 1, 2), solution);
    }
    #[test]
    fn flood_fill2() {
        let solution = vec![vec![0, 0, 0], vec![0, 0, 0]];
        let image = vec![vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(flood_fill::flood_fill(image, 0, 0, 0), solution);
    }
    #[test]
    fn lowest_common_ancestor1() {
        // Test case 1: Example tree
        //        3
        //       / \
        //      5   1
        //     / \ / \
        //    6  2 0  8
        //      / \
        //     7   4
        let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let node5 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let node1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let node6 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let node0 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let node8 = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let node7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let node4 = Some(Rc::new(RefCell::new(TreeNode::new(4))));

        root.as_ref().unwrap().borrow_mut().left = node5.clone();
        root.as_ref().unwrap().borrow_mut().right = node1.clone();
        node5.as_ref().unwrap().borrow_mut().left = node6.clone();
        node5.as_ref().unwrap().borrow_mut().right = node2.clone();
        node2.as_ref().unwrap().borrow_mut().left = node7.clone();
        node2.as_ref().unwrap().borrow_mut().right = node4.clone();
        node1.as_ref().unwrap().borrow_mut().left = node0.clone();
        node1.as_ref().unwrap().borrow_mut().right = node8.clone();

        assert_eq!(
            lowest_common_ancestor::lowest_common_ancestor(
                root.clone(),
                node5.clone(),
                node1.clone()
            ),
            root
        );
    }
    #[test]
    fn balanced_binary_tree1() {
        // Test case 1: Example tree
        //        3
        //       / \
        //      5   1
        //     / \ / \
        //    6  2 0  8
        //      / \
        //     7   4
        let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let node5 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let node1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let node6 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let node0 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let node8 = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let node7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let node4 = Some(Rc::new(RefCell::new(TreeNode::new(4))));

        root.as_ref().unwrap().borrow_mut().left = node5.clone();
        root.as_ref().unwrap().borrow_mut().right = node1.clone();
        node5.as_ref().unwrap().borrow_mut().left = node6.clone();
        node5.as_ref().unwrap().borrow_mut().right = node2.clone();
        node2.as_ref().unwrap().borrow_mut().left = node7.clone();
        node2.as_ref().unwrap().borrow_mut().right = node4.clone();
        node1.as_ref().unwrap().borrow_mut().left = node0.clone();
        node1.as_ref().unwrap().borrow_mut().right = node8.clone();

        assert_eq!(balanced_binary_tree::is_balanced(root.clone()), true);
    }
    #[test]
    fn platesbetweencandles1() {
        let queries = vec![vec![3, 8]];
        let s = String::from("||**||**|*");
        let res = vec![2];
        assert_eq!(
            res,
            plates_between_candles::plates_between_candles(s, queries)
        );
    }
    #[test]
    fn platesbetweencandles2() {
        let queries = vec![vec![2, 5], vec![5, 9]];
        let s = String::from("**|**|***|");
        let res = vec![2, 3];
        assert_eq!(
            res,
            plates_between_candles::plates_between_candles(s, queries)
        );
    }
    #[test]
    fn platesbetweencandles3() {
        let queries = vec![
            vec![1, 17],
            vec![4, 5],
            vec![14, 17],
            vec![5, 11],
            vec![15, 16],
        ];
        let s = String::from("***|**|*****|**||**|*");
        let res = vec![9, 0, 0, 0, 0];
        assert_eq!(
            res,
            plates_between_candles::plates_between_candles(s, queries)
        );
    }
}
