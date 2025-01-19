use blake3::Hasher;
use std::fmt;

// MerkleTree struct definition
pub struct MerkleTree {
    root: Option<[u8; 32]>,
    depth: usize,
}

impl MerkleTree {
    // Create a new Merkle Tree
    pub fn new(data: Vec<&str>) -> Self {
        if data.is_empty() {
            return Self {
                root: None,
                depth: 0,
            };
        }

        let mut nodes: Vec<[u8; 32]> = data
            .iter()
            .map(|item| {
                let mut hasher = Hasher::new();
                hasher.update(item.as_bytes());
                hasher.finalize().into()
            })
            .collect();

        let mut depth = 0;

        // Build the tree up to the root
        while nodes.len() > 1 {
            nodes = nodes
                .chunks(2)
                .map(|chunk| {
                    let mut hasher = Hasher::new();
                    hasher.update(&chunk[0]);

                    if chunk.len() > 1 {
                        hasher.update(&chunk[1]);
                    }

                    hasher.finalize().into()
                })
                .collect();

            depth += 1;
        }

        // The root node is the last remaining hash
        Self {
            root: Some(nodes[0]),
            depth,
        }
    }

    // Return the Merkle root
    pub fn root(&self) -> Option<[u8; 32]> {
        self.root
    }
}

impl fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(root) = self.root {
            write!(f, "Merkle Root: {}", hex::encode(root))
        } else {
            write!(f, "Empty Merkle Tree")
        }
    }
}

// Test the MerkleTree
fn main() {
    let data = vec!["a", "b", "c", "d"];
    let merkle_tree = MerkleTree::new(data);

    println!("Merkle Tree: {}", merkle_tree);
}
