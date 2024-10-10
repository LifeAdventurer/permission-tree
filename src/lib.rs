use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    Public,
    Private,
}

#[derive(Debug)]
pub struct TreeNode {
    pub id: u32,
    pub permission: Permission,
    pub children: HashSet<u32>,
}

#[derive(Debug)]
pub struct Tree {
    pub nodes: HashMap<u32, TreeNode>,
    pub parent_map: HashMap<u32, u32>, // Keeps track of parent-child relationships
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: HashMap::new(),
            parent_map: HashMap::new(),
        }
    }

    // Add a node with permission to the tree
    pub fn add_node(&mut self, id: u32, permission: Permission) {
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
    pub fn connect_nodes(&mut self, parent_id: u32, child_id: u32) {
        if !self.nodes.contains_key(&parent_id) || !self.nodes.contains_key(&child_id) {
            println!("Either parent or child node doesn't exist");
            return;
        }

        // Check if the parent ID and child ID are the same
        if parent_id == child_id {
            println!("A node cannot be its own parent");
            return;
        }

        // Check if the child already has a parent
        if self.parent_map.contains_key(&child_id) {
            println!("Node {} already has a parent", child_id);
            return;
        }

        if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
            parent_node.children.insert(child_id);
            self.parent_map.insert(child_id, parent_id);
            println!("Node {} connected as child of {}", child_id, parent_id);
        }

        // Ensure the child inherits permission if the parent is private
        self.update_permission(child_id);
    }

    pub fn is_descendant(&self, node_id: u32, potential_descendant_id: u32) -> bool {
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
    pub fn move_subtree(&mut self, node_id: u32, new_parent_id: u32) {
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

    pub fn print_tree(&self, root: u32, indent: usize) -> String {
        let mut result = String::new();

        if let Some(node) = self.nodes.get(&root) {
            result.push_str(&format!(
                "{:indent$}- node {} ({:?})",
                "",
                node.id,
                node.permission,
                indent = indent
            ));
            for &child in &node.children {
                result.push_str(&self.print_tree(child, indent + 4));
            }
        }
        result
    }
}
