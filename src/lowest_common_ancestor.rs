use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

/**
 * Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes
 * in the BST.
 *
 * According to the definition of LCA on Wikipedia: "The lowest common ancestor is defined between
 * two nodes p and q as the lowest node in T that has both p and q as descendents (where we allow a
 * node to be a descendent of itself)."
 */
pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, 
                              q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>{
    if let Some(refer) = root.as_ref(){
        let node = refer.borrow();
        match (p.as_ref(),q.as_ref()) {
            (Some(pn), Some(qn)) => {
                let pnode = pn.borrow();
                let qnode = qn.borrow();
                if (node.val == pnode.val) || (node.val == qnode.val) {
                    return Some(refer.clone());
                }
                else if (pnode.val > node.val) && (qnode.val > node.val) {
                    return lowest_common_ancestor(node.right.clone(), Some(pn.clone()), Some(qn.clone()));
                }
                else if (pnode.val < node.val) && (qnode.val < node.val) {
                    return lowest_common_ancestor(node.left.clone(), Some(pn.clone()), Some(qn.clone()));
                }
                else {
                    return Some(refer.clone()); 
                }
            },
            _ => return Some(refer.clone()),
        }
    }
    root 
}
