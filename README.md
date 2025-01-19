# Merkle Tree Implementation Using BLAKE3

This project implements a **Merkle Tree** using the **BLAKE3 hashing algorithm** in Rust. The implementation allows users to generate a Merkle Tree from a set of data inputs, compute the Merkle root, and display it in a human-readable hexadecimal format.

## Features

- **Efficient Merkle Tree Construction**: Builds a Merkle Tree from a list of input data.
- **BLAKE3 Hashing**: Utilizes the high-performance and secure BLAKE3 hashing algorithm.
- **Hexadecimal Representation**: Displays the Merkle root in a readable hex format.
- **Dynamic Depth Calculation**: Automatically calculates the depth of the tree.

## Requirements

To run this project, ensure you have the following:

- **Rust** (latest stable version)
- Cargo (comes with Rust)

## Dependencies

The project uses the following Rust crates:

- **blake3**: For hashing data using the BLAKE3 algorithm.
- **hex**: For encoding hash outputs into hexadecimal strings.

Add these dependencies to your `Cargo.toml` file:

```toml
[dependencies]
blake3 = "1.3"
hex = "0.4"
```

## Usage

### Clone the Repository

```bash
git clone <repository_url>
cd <repository_folder>
```

### Build the Project

Use Cargo to build the project:

```bash
cargo build
```

### Run the Project

Run the project using Cargo:

```bash
cargo run
```

### Example Output

When running the project with sample data (`["a", "b", "c", "d"]`), you should see:

```text
Merkle Tree: Merkle Root: <hexadecimal hash value>
```

## Code Overview

### MerkleTree Struct

The `MerkleTree` struct is the core of this implementation. It provides methods for constructing the tree and retrieving the Merkle root.

#### Methods

- **`new(data: Vec<&str>) -> Self`**:
  - Constructs a new Merkle Tree from the input data.
  - Each piece of data is hashed using BLAKE3, and parent hashes are computed iteratively until the root is obtained.

- **`root(&self) -> Option<[u8; 32]>`**:
  - Returns the Merkle root as a byte array.

#### Display Implementation

The `Display` trait is implemented for `MerkleTree`, allowing the Merkle root to be displayed as a hexadecimal string.

### Main Function

The `main` function demonstrates the usage of the `MerkleTree` struct with sample data. You can modify the data set to test with different inputs.

## Customization

You can customize the project as needed:

- **Input Data**:
  - Replace the sample data in the `main` function with your own data.
- **Hashing Algorithm**:
  - Although this implementation uses BLAKE3, you can modify the code to use other hashing algorithms if required.

## Advantages of BLAKE3

- **Speed**: BLAKE3 is faster than traditional algorithms like SHA-2.
- **Parallelism**: Ideal for modern multi-core systems.
- **Security**: Provides cryptographic security suitable for Merkle Trees.

## Applications

The Merkle Tree implementation with BLAKE3 can be used in:

1. **Blockchain Systems**: To validate transactions and compute block headers.
2. **Distributed File Storage**: For data integrity verification.
3. **Secure Logs**: To create tamper-evident logging systems.
4. **File Integrity Checks**: To verify that files have not been altered.

## License

This project is open-source and available under the MIT License. Feel free to use and modify it as needed.

## Contributing

Contributions are welcome! If you encounter issues or have suggestions, please open an issue or submit a pull request.

## Contact

For any questions or support, please contact [Your Name] at [Your Email Address].

