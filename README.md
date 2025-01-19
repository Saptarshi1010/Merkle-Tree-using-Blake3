# Merkle Tree Implementation Using BLAKE3

This project implements a **Merkle Tree** using the **BLAKE3 hashing algorithm** in Rust. The implementation allows users to generate a Merkle Tree from a set of data inputs, compute the Merkle root, and display it in a human-readable hexadecimal format.

## Features

- **Efficient Merkle Tree Construction**: Builds a Merkle Tree from a list of input data.
- **BLAKE3 Hashing**: Utilizes the high-performance and secure BLAKE3 hashing algorithm.
- **Hexadecimal Representation**: Displays the Merkle root in a readable hex format.
- **Dynamic Depth Calculation**: Automatically calculates the depth of the tree.

## Why use of BLAKE3

- **Speed**: BLAKE3 is faster than traditional algorithms like SHA-2.
- **Parallelism**: Ideal for modern multi-core systems.
- **Security**: Provides cryptographic security suitable for Merkle Trees.
  
## Requirements

To run this project, ensure you have the following:

- **Rust** (latest stable version)
- Cargo (comes with Rust)

## Dependencies

The project uses the following Rust crates:

- **blake3**: For hashing data using the BLAKE3 algorithm.
- **hex**: For encoding hash outputs into hexadecimal strings.

Add these dependencies to your `Cargo.toml` file:

```
[dependencies]
blake3 = "1.3"
hex = "0.4"
```

### Clone the Repository

```
git clone <repository_url>
cd <repository_folder>
```

### Build the Project

Use Cargo to build the project:

```
cargo build
```

### Run the Project

Run the project using Cargo:

```
cargo run
```

### Example Output

When running the project with sample data (`["a", "b", "c", "d"]`), you should see:

```
Merkle Tree: Merkle Root: <hexadecimal hash value>
```

## Contributing

Contributions are welcome! If you encounter issues or have suggestions, please open an issue or submit a pull request.


