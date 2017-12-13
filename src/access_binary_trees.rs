//! Rust implementation of access-binary-trees.
// Original implementation from The Great Computer Language Shootout,
// contributed by Isaac Gouy.

use std::cmp;

// A TreeNode either contains zero or two children.
struct TreeNode {
    children: Option<(Box<TreeNode>, Box<TreeNode>)>,
    item: i32,
}

impl TreeNode {
    pub fn item_check(&self) -> i32 {
        match self.children {
            None => self.item,
            Some((ref left, ref right)) =>
                self.item + left.item_check() - right.item_check()
        }
    }
}

fn bottom_up_tree(item: i32, depth: u32) -> Box<TreeNode> {
    if depth > 0 {
        let left = bottom_up_tree(2*item-1, depth-1);
        let right = bottom_up_tree(2*item, depth-1);

        Box::new(TreeNode { children: Some((left, right)), item: item })
    } else {
        Box::new(TreeNode { children: None, item: item })
    }
}

#[no_mangle]
pub fn access_binary_trees() -> i32 {
    let mut ret: i32 = 0;

    for n in 4..8 {
        let min_depth = 4;
        let max_depth = cmp::max(min_depth + 2, n);
        let stretch_depth = max_depth + 1;
        
        bottom_up_tree(0, stretch_depth).item_check();

        let long_lived_tree = bottom_up_tree(0, max_depth);
        for depth in (min_depth .. max_depth+1).step_by(2) {
            let iterations = 1 << (max_depth - depth + min_depth);

            for i in 1 .. iterations+1 {
                bottom_up_tree(i, depth).item_check();
                bottom_up_tree(-i, depth).item_check();
            }
        }

        ret += long_lived_tree.item_check();
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_binary_trees() {
        assert_eq!(access_binary_trees(), -4);
    }
}
