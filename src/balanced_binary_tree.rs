use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;
use std::cmp;
/**
 * Given a  binary tree, determine if it is height balanced.
 * A height-balanced binary tree is a binary tree in which the depth of the two subtrees of every
 * node never differs by more than one.
 */
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(node) = root.as_ref() {
        let rnode = node.borrow();
        match(rnode.left.as_ref(), rnode.left.as_ref()) {
            (Some(l),Some(r)) => {
                    return is_balanced(Some(l.clone())) && is_balanced(Some(r.clone()));
                },
            (Some(l),None) => {
                    if depth(Some(l.clone())) > 1 {
                        return false;
                    }
                },
            (None,Some(r)) => {
                    if depth(Some(r.clone())) > 1 {
                        return false;
                    }
                },
            _ =>return true,
        }
    }
    true
}

fn depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 { 
    if let Some(node) = root.as_ref() {
        let rnode = node.borrow();
        match(rnode.left.as_ref(), rnode.left.as_ref()) {
            (Some(l),Some(r)) => {
                    return 1 + cmp::max(depth(Some(l.clone())),depth(Some(r.clone())));
                },
            (Some(l),None) => {
                    return 1 + depth(Some(l.clone()));
                },
            (None,Some(r)) => {
                    return 1 + depth(Some(r.clone()));
                },
            _ =>return 1,
        }
    }
    0
}
