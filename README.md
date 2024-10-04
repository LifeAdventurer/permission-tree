# Permission Tree

A tree structure that manages permissions and allows moving subtrees. This project implements a simple permission tree where each node can be public or private. When a public subtree is moved to a private node, the entire subtree becomes private.

## Features

- Create and manage a tree structure.
- Add nodes with customizable permissions (public/private).
- Move subtrees while maintaining permission rules.
- Inherit permissions from parent nodes.

## Getting Started

### Prerequisites

Ensure you have [Rust](https://www.rust-lang.org/) installed on your machine. You can install it using `rustup`:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/permission-tree.git
    cd permission-tree
    ```

2. Build the project:
    
    ```sh
    cargo build
    ```

3. Run the project:

    ```sh
    cargo run
    ```

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0). See the [LICENSE](./LICENSE) file for more details.
