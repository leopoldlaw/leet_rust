use crate::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::DerefMut;


/**
* Given the root of a binary tree, invert the tree and return its root.
 */
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>{ 
    // as_ref gives me an Option<&T>
    if let Some(node) = root.as_ref() {
        
        //borrow_mut gives me a mutable borrowed reference for mutating the root
        let mut mutRoot = node.borrow_mut();
        
        // Need to clone as the mutable reference to the smart pointer
        // is only valid within this scope (the pointed to memory is not cloned)
        invert_tree(mutRoot.left.clone());
        invert_tree(mutRoot.right.clone());
        
        // We want a mutable reference to the TreeNode we're mutating
        // We need to dereference the borrowed reference, then qualify it
        let mut x = mutRoot.deref_mut(); // Same as &mut *mutRoot;
        std::mem::swap(&mut x.left,&mut x.right);
    }
    root
}
