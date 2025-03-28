use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode {
    pub id: u32,
    pub permission: Permission,
    pub children: HashSet<u32>,
    pub tags: Option<HashSet<String>>,
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

    // Add a node with permission to the tree. Tags are set to None initially
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
                tags: None,
            },
        );
        println!("Node with ID {} added with {:?} permission", id, permission);
    }

    // Add a tag to a node; initialize tags if they are None.
    pub fn add_tag_to_node(&mut self, id: u32, tag: String) {
        if let Some(node) = self.nodes.get_mut(&id) {
            match &mut node.tags {
                Some(tags) => {
                    tags.insert(tag);
                }
                None => {
                    let mut new_tags = HashSet::new();
                    new_tags.insert(tag);
                    node.tags = Some(new_tags);
                }
            }
            // Optionally, update tags for the subtree to reflect inherited changes.
            self.update_tags(id);
        } else {
            println!("Node with ID {} does not exist", id);
        }
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

        // Update both permission and tags to inherit from the parent
        self.update_permission(child_id);
        self.update_tags(child_id);
    }

    pub fn is_descendant(&self, node_id: u32, potential_descendant_id: u32) -> bool {
        // Start with the potential descendant
        let mut current_id = potential_descendant_id;

        // Traverse up the parent chain
        while let Some(&parent_id) = self.parent_map.get(&current_id) {
            if parent_id == node_id {
                return true; // Found the ancestor
            }
            current_id = parent_id;
        }

        false // If we reached the root without finding the node
    }

    // Move a subtree rooted at `node_id` under `new_parent_id`
    pub fn move_subtree(&mut self, node_id: u32, new_parent_id: u32) {
        if !self.nodes.contains_key(&node_id) || !self.nodes.contains_key(&new_parent_id) {
            println!("Either node or new parent doesn't exist");
            return;
        }

        // Prevent moving a node into its own subtree
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

        // Update both permissions and tags for the moved subtree based on the new parent
        self.update_permission(node_id);
        self.update_tags(node_id);

        println!(
            "Moved subtree rooted at node {} to new parent node {}",
            node_id, new_parent_id
        );
    }

    // Recursively update the permission of a node and its subtree.
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

    // Recursively update the tags of a node and its subtree by inheriting parent's tags.
    fn update_tags(&mut self, node_id: u32) {
        // Retrieve inherited tags from the parent (if any)
        let inherited_tags = if let Some(&parent_id) = self.parent_map.get(&node_id) {
            if let Some(parent_node) = self.nodes.get(&parent_id) {
                parent_node.tags.clone().unwrap_or_else(HashSet::new)
            } else {
                HashSet::new()
            }
        } else {
            HashSet::new()
        };

        // Update current node's tags: merge its own tags (if present) with inherited ones.
        if let Some(node) = self.nodes.get_mut(&node_id) {
            match &mut node.tags {
                Some(tags_set) => {
                    // Insert inherited tags into the existing set (set union is idempotent)
                    tags_set.extend(inherited_tags);
                }
                None => {
                    if !inherited_tags.is_empty() {
                        node.tags = Some(inherited_tags);
                    }
                }
            }
        }

        // Recursively update tags for all children
        if let Some(node) = self.nodes.get(&node_id) {
            let children: Vec<u32> = node.children.iter().cloned().collect();
            for child_id in children {
                self.update_tags(child_id);
            }
        }
    }

    // Print the tree starting from the given root.
    pub fn print_tree(&self, root: u32, indent: usize) -> String {
        let mut result = String::new();

        if let Some(node) = self.nodes.get(&root) {
            result.push_str(&format!(
                "{:indent$}- node {} ({:?}), tags: {:?}\n",
                "",
                node.id,
                node.permission,
                node.tags,
                indent = indent
            ));
            for &child in &node.children {
                result.push_str(&self.print_tree(child, indent + 4));
            }
        }
        result
    }
}
