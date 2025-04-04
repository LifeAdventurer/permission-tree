use permission_tree::{Permission, Tree};

#[test]
fn test_add_node() {
    let mut tree = Tree::new();
    tree.add_node(1, Permission::Public);

    // Assert that node with ID 1 has been added.
    assert!(tree.nodes.contains_key(&1));
    let node = tree.nodes.get(&1).unwrap();
    assert_eq!(node.permission, Permission::Public);
    // tags should be None by default.
    assert!(node.tags.is_none());
}

#[test]
fn test_add_tag_to_node() {
    let mut tree = Tree::new();
    tree.add_node(1, Permission::Public);

    // Initially no tags.
    assert!(tree.nodes.get(&1).unwrap().tags.is_none());

    // Add a tag.
    tree.add_tag_to_node(1, "root_tag".to_string());

    // Now, node should have tags.
    let node = tree.nodes.get(&1).unwrap();
    assert!(node.tags.is_some());
    let tags = node.tags.as_ref().unwrap();
    assert!(tags.contains("root_tag"));
}

#[test]
fn test_tags_inheritance_on_connect() {
    let mut tree = Tree::new();

    // Adding nodes
    tree.add_node(1, Permission::Public); // parent node
    tree.add_node(2, Permission::Public); // child node

    // Add a tag to the parent node.
    tree.add_tag_to_node(1, "parent_tag".to_string());

    // Connect nodes; child should inherit parent's tag.
    tree.connect_nodes(1, 2);

    let child_tags = tree.nodes.get(&2).unwrap().tags.as_ref().unwrap();
    assert!(child_tags.contains("parent_tag"));
}

#[test]
fn test_tags_inheritance_with_existing_child_tags() {
    let mut tree = Tree::new();

    // Adding nodes
    tree.add_node(1, Permission::Public); // parent node
    tree.add_node(2, Permission::Public); // child node

    // Add tags to both parent and child.
    tree.add_tag_to_node(1, "parent_tag".to_string());
    tree.add_tag_to_node(2, "child_tag".to_string());

    // Connect nodes; child's tags should be the union of its own and the parent's.
    tree.connect_nodes(1, 2);

    let child_tags = tree.nodes.get(&2).unwrap().tags.as_ref().unwrap();
    assert!(child_tags.contains("parent_tag"));
    assert!(child_tags.contains("child_tag"));
}

#[test]
fn test_tags_in_move_subtree() {
    let mut tree = Tree::new();

    // Adding nodes
    tree.add_node(1, Permission::Public); // root node
    tree.add_node(2, Permission::Public);
    tree.add_node(3, Permission::Public);

    // Connect nodes: 1->2 and 1->3.
    tree.connect_nodes(1, 2);
    tree.connect_nodes(1, 3);

    // Add a tag to node 3 (the new parent)
    tree.add_tag_to_node(3, "new_parent_tag".to_string());
    // Also add a tag to node 2 (child)
    tree.add_tag_to_node(2, "child_tag".to_string());

    // Move subtree: move node 2 to be a child of node 3.
    tree.move_subtree(2, 3);

    // After moving, node 2 should have both its own tag and inherit node 3's tag.
    let child_tags = tree.nodes.get(&2).unwrap().tags.as_ref().unwrap();
    assert!(child_tags.contains("new_parent_tag"));
    assert!(child_tags.contains("child_tag"));
}

#[test]
fn test_connect_nodes() {
    let mut tree = Tree::new();

    // Adding nodes
    tree.add_node(1, Permission::Public); // root node
    tree.add_node(2, Permission::Private);
    tree.add_node(3, Permission::Public);

    // Connecting nodes
    tree.connect_nodes(1, 2);
    tree.connect_nodes(1, 3);

    // Check if the connections are established correctly
    assert!(tree.nodes.get(&1).unwrap().children.contains(&2));
    assert!(tree.nodes.get(&1).unwrap().children.contains(&3));

    // Attempt to connect a node to itself (should fail)
    tree.connect_nodes(1, 1);
    assert_eq!(tree.nodes.get(&1).unwrap().children.contains(&1), false);

    // Attempt to connect node 2 to node 3, which should fail
    tree.connect_nodes(2, 3);
    assert_eq!(tree.nodes.get(&2).unwrap().children.contains(&3), false);
    assert_eq!(tree.parent_map.get(&3), Some(&1));
}

#[test]
fn test_permission_inheritance() {
    let mut tree = Tree::new();

    // Adding nodes
    tree.add_node(1, Permission::Public); // root node
    tree.add_node(2, Permission::Public);
    tree.add_node(3, Permission::Private); // private node
    tree.add_node(4, Permission::Public);
    tree.add_node(5, Permission::Public);

    // Connecting nodes
    tree.connect_nodes(1, 3);
    tree.connect_nodes(2, 4);
    tree.connect_nodes(2, 5);

    // Before the connection, node 4 and node 5 should be public
    assert_eq!(tree.nodes.get(&4).unwrap().permission, Permission::Public);
    assert_eq!(tree.nodes.get(&5).unwrap().permission, Permission::Public);

    // Connecting node 2 (public) as a child of node 3 (private)
    tree.connect_nodes(3, 2);

    // After connection, node 2 and all its descendants (4, 5) should become private
    assert_eq!(tree.nodes.get(&2).unwrap().permission, Permission::Private);
    assert_eq!(tree.nodes.get(&4).unwrap().permission, Permission::Private);
    assert_eq!(tree.nodes.get(&5).unwrap().permission, Permission::Private);
}

#[test]
fn test_is_descendant() {
    let mut tree = Tree::new();

    // Adding nodes
    tree.add_node(1, Permission::Public); // root node
    tree.add_node(2, Permission::Public);
    tree.add_node(3, Permission::Public);
    tree.add_node(4, Permission::Public);

    // Connecting nodes
    tree.connect_nodes(1, 2);
    tree.connect_nodes(2, 3);
    tree.connect_nodes(3, 4);

    // Test if node 4 is a descendant of node 1
    assert!(tree.is_descendant(1, 4));
    // Test if node 3 is a descendant of node 1
    assert!(tree.is_descendant(1, 3));

    // Test if node 2 is not a descendant of node 4
    assert!(tree.is_descendant(1, 2));
}

#[test]
fn test_move_subtree() {
    let mut tree = Tree::new();

    // Adding nodes
    tree.add_node(1, Permission::Public); // root node
    tree.add_node(2, Permission::Public);
    tree.add_node(3, Permission::Private); // private node
    tree.add_node(4, Permission::Public);
    tree.add_node(5, Permission::Public);

    // Connecting nodes
    tree.connect_nodes(1, 2);
    tree.connect_nodes(1, 3);
    tree.connect_nodes(2, 4);
    tree.connect_nodes(2, 5);

    // Move subtree rooted at node 2 under node 3 (private)
    tree.move_subtree(2, 3);

    // After moving, nodes 2, 4, and 5 should inherit private permission from node 3
    assert_eq!(tree.nodes.get(&2).unwrap().permission, Permission::Private);
    assert_eq!(tree.nodes.get(&4).unwrap().permission, Permission::Private);
    assert_eq!(tree.nodes.get(&5).unwrap().permission, Permission::Private);
}

#[test]
fn test_move_subtree_invalid() {
    let mut tree = Tree::new();

    // Adding nodes
    tree.add_node(1, Permission::Public); // root node
    tree.add_node(2, Permission::Public);
    tree.add_node(3, Permission::Private); // private node
    tree.add_node(4, Permission::Public);

    // Connecting nodes
    tree.connect_nodes(1, 2);
    tree.connect_nodes(2, 3);
    tree.connect_nodes(3, 4);

    // Try to move node 3 under node 4, which is its descendant (should fail).
    tree.move_subtree(3, 4);

    // After moving, nodes 2, 4, and 5 should inherit private permission from node 3.
    assert!(tree.nodes.get(&2).unwrap().children.contains(&3));
    assert!(!tree.nodes.get(&4).unwrap().children.contains(&3));
}
