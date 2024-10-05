use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
enum Permission {
    Public,
    Private,
}

#[derive(Debug)]
struct TreeNode {
    id: u32,
    permission: Permission,
    children: HashSet<u32>,
}

#[derive(Debug)]
struct Tree {
    nodes: HashMap<u32, TreeNode>,
    parent_map: HashMap<u32, u32>, // Keeps track of parent-child relationships
}

impl Tree {
    fn new() -> Self {
        Tree {
            nodes: HashMap::new(),
            parent_map: HashMap::new(),
        }
    }

    // Add a node with permission to the tree
    fn add_node(&mut self, id: u32, permission: Permission) {
        if self.nodes.contains_key(&id) {
            println!("Node with ID {} already exists", id);
            return;
        }
        self.nodes.insert(
            id,
            TreeNode {
                id,
                permission: permission.clone(),
                children: HashSet::new(),
            },
        );
        println!("Node with ID {} added with {:?} permission", id, permission);
    }

    // Connect two nodes, making `parent_id` the parent of `child_id`
    fn connect_nodes(&mut self, parent_id: u32, child_id: u32) {
        if !self.nodes.contains_key(&parent_id) || !self.nodes.contains_key(&child_id) {
            println!("Either parent or child node doesn't exist");
            return;
        }

        if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
            parent_node.children.insert(child_id);
            self.parent_map.insert(child_id, parent_id);
            println!("node {} connected as child of {}", child_id, parent_id);
        }

        // Ensure the child inherits permission if the parent is private
        self.update_permission(child_id);
    }

    fn is_descendant(&self, node_id: u32, potential_descendant_id: u32) -> bool {
        if let Some(node) = self.nodes.get(&node_id) {
            if node.children.contains(&potential_descendant_id) {
                return true;
            }
            for &child_id in &node.children {
                if self.is_descendant(child_id, potential_descendant_id) {
                    return true;
                }
            }
        }
        false
    }

    // Move a subtree rooted at `note_id` under `new_parent_id`
    fn move_subtree(&mut self, node_id: u32, new_parent_id: u32) {
        if !self.nodes.contains_key(&node_id) || !self.nodes.contains_key(&new_parent_id) {
            println!("Either node or new parent doesn't exists");
            return;
        }

        // Prevent moving a node into it's own subtree
        if self.is_descendant(node_id, new_parent_id) {
            println!("Cannot move a node into its own subtree");
            return;
        }

        // Find the current parent of `node_id`
        if let Some(&current_parent_id) = self.parent_map.get(&node_id) {
            // Remove node_id from current parent's children
            if let Some(current_parent) = self.nodes.get_mut(&current_parent_id) {
                current_parent.children.remove(&node_id);
            }
        }

        // Move the node to the new parent
        if let Some(new_parent_node) = self.nodes.get_mut(&new_parent_id) {
            new_parent_node.children.insert(node_id);
        }
        self.parent_map.insert(node_id, new_parent_id);

        // Update permissions for the subtree based on the new parent
        self.update_permission(node_id);

        println!(
            "Moved subtree rooted at node {} to new parent node {}",
            node_id, new_parent_id
        )
    }

    // Recursively update the permission of a node and its subtree
    fn update_permission(&mut self, node_id: u32) {
        if let Some(node) = self.nodes.get(&node_id) {
            // If this node is private, its entire subtree must be private
            if node.permission == Permission::Private {
                // No need to continue if this node is private
                return;
            }
        }

        if let Some(&parent_id) = self.parent_map.get(&node_id) {
            if let Some(parent_node) = self.nodes.get(&parent_id) {
                // If parent is private, make the node private as well
                if parent_node.permission == Permission::Private {
                    if let Some(node) = self.nodes.get_mut(&node_id) {
                        node.permission = Permission::Private;
                    }
                }
            }
        }

        // Recursively update permission of all children
        if let Some(node) = self.nodes.get(&node_id) {
            for child_id in node.children.clone() {
                self.update_permission(child_id);
            }
        }
    }

    fn print_tree(&self, root: u32, indent: usize) {
        if let Some(node) = self.nodes.get(&root) {
            println!(
                "{:indent$}- node {} ({:?})",
                "",
                node.id,
                node.permission,
                indent = indent
            );
            for &child in &node.children {
                self.print_tree(child, indent + 4);
            }
        }
    }
}

fn main() {
    let mut tree = Tree::new();

    // Adding nodes with permission
    tree.add_node(1, Permission::Public); // root node
    tree.add_node(2, Permission::Public);
    tree.add_node(3, Permission::Private);
    tree.add_node(4, Permission::Public);
    tree.add_node(5, Permission::Public);
    tree.add_node(6, Permission::Public);

    // Connecting nodes
    tree.connect_nodes(1, 2);
    tree.connect_nodes(1, 3);
    tree.connect_nodes(2, 4);
    tree.connect_nodes(2, 5);

    // After this connection, node 6 will inherit the private permission from node 3,
    // meaning that node 6 will also become private despite being public before.
    tree.connect_nodes(3, 6);

    println!("\nInitial tree:");
    tree.print_tree(1, 0);

    // Moving a public subtree (node 2 and its children) to a private node (node 3))
    tree.move_subtree(2, 3);

    println!("\nTree after moving subtree rooted at node 2 under node 3:");
    tree.print_tree(1, 0);

    // Attempting to move node 3 under node 6, which should fail since 6 is a descendant of 3
    tree.move_subtree(3, 6);
}
