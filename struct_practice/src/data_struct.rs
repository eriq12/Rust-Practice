struct BinaryTree <T> {
    data : T,
    left_node : BinaryTree<T>,
    right_node : BinaryTree<T>,
}

fn build_binary_tree(data : T, left_node : BinaryTree<T>, right_node : BinaryTree<T>) -> BinaryTree<T> {
    // can do data: data, but could also do the init shorthand as listed below
    BinaryTree<T> {
        data,
        left_node,
        right_node,
    }
}