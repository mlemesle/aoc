use lib::btree::BTree;

#[test]
fn get_root() {
    let (tree, _) = BTree::new("Yay");

    insta::assert_debug_snapshot!(tree);
}

#[test]
fn add_left() {
    let (mut tree, root) = BTree::new("I'm root");

    let left_child = tree.add_left("Left child", root);
    tree.add_left("Left child's left child", left_child);

    insta::assert_debug_snapshot!(tree);
}

#[test]
fn add_right() {
    let (mut tree, root) = BTree::new("I'm root");

    let right_child = tree.add_right("Right child", root);
    tree.add_right("Right child's right child", right_child);

    insta::assert_debug_snapshot!(tree);
}

#[test]
fn remove_leaf() {
    let (mut tree, root) = BTree::new("I'm root");

    let left_child = tree.add_left("Left child", root);
    tree.remove(left_child);

    insta::assert_debug_snapshot!(tree);
}

#[test]
fn remove_sub_tree() {
    let (mut tree, root) = BTree::new("I'm root");

    let left_child = tree.add_left("Left child", root);
    tree.add_left("Left child's left child", left_child);
    tree.add_right("Left child's right child", left_child);

    tree.remove(left_child);

    insta::assert_debug_snapshot!(tree);
}

#[test]
fn remove_sub_tree_then_add_new_nodes() {
    let (mut tree, root) = BTree::new("I'm root");

    let left_child = tree.add_left("Left child", root);
    tree.add_left("Left child's left child", left_child);
    tree.add_right("Left child's right child", left_child);

    tree.remove(left_child);

    let new_right_child = tree.add_right("New right child", root);
    tree.add_left("New left child for new right child", new_right_child);

    insta::assert_debug_snapshot!(tree);
}
