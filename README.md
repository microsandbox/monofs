<div align="center">
  <h1 align="center">monofs</h1>

  <p>
    <a href="https://discord.gg/T95Y3XnEAK">
      <img src="https://img.shields.io/static/v1?label=Discord&message=join%20us!&color=mediumslateblue&logo=discord&logoColor=white" alt="Discord">
    </a>
    <a href="https://github.com/appcypher/monocore/actions?query=">
      <img src="https://github.com/appcypher/monocore/actions/workflows/tests_and_checks.yml/badge.svg" alt="Build Status">
    </a>
    <a href="https://crates.io/crates/monofs">
      <img src="https://img.shields.io/crates/v/monofs?label=crates&logo=rust" alt="Monofs Crate">
    </a>
    <a href="https://docs.rs/monofs">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue&logo=docs.rs" alt="Monofs Docs">
    </a>
    <a href="https://github.com/appcypher/monocore/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg?logo=apache&logoColor=white" alt="License">
    </a>
  </p>
</div>

**`monofs`** is a versioned content-addressed filesystem designed for distributed applications. It is based largely on the [WNFS](https://github.com/wnfs-wg/rs-wnfs) public filesystem.

> [!WARNING]
> This project is in early development and is not yet ready for production use.

##

## ✨ Features

- 🔄 **Automatic Deduplication**: Save storage space by storing identical content only once, even across different files and directories
- 🔒 **Versioned**: Every change creates a new version, making it impossible to accidentally lose data
- 🌐 **Built for Distribution**: Perfect for peer-to-peer and decentralized applications with content-addressed storage
- ⚡ **Efficient Syncing**: Only transfer what's changed between versions, saving bandwidth and time
- 🛡️ **Data Integrity**: Content addressing ensures data hasn't been tampered with or corrupted

## 🚀 Getting Started

### Installation

```sh
curl -sSfL https://get.monofs.dev | sh
```

### Quick Start

TODO: Demo of running multiple servers on different paths syncing up with each other and use with monocore.

### API

#### Working with Files

```rust
use monofs::filesystem::File;
use ipldstore::MemoryStore;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let store = MemoryStore::default();

    // Create a new file
    let mut file = File::new(store.clone());

    // Write content to the file
    let mut output_stream = file.get_output_stream();
    output_stream.write_all(b"Hello, monofs!").await?;
    output_stream.shutdown().await?;

    // Read content from the file
    let mut input_stream = file.get_input_stream().await?;
    let mut buffer = Vec::new();
    input_stream.read_to_end(&mut buffer).await?;

    println!("File content: {}", String::from_utf8_lossy(&buffer));

    // Drop reader to free up reference to the file
    drop(input_stream);

    // Persist changes; creates a new version of the file
    let file_cid = file.checkpoint().await?;
    println!("Checkpoint file with CID: {}", file_cid);

    Ok(())
}
```

#### Working with Directories

```rust
use monofs::filesystem::{Dir, FsResult};
use ipldstore::MemoryStore;

#[tokio::main]
async fn main() -> FsResult<()> {
    let store = MemoryStore::default();

    // Create a new root directory
    let mut root = Dir::new(store.clone());

    // Create a file in the directory
    root.create_file("example.txt").await?;

    // Create a subdirectory
    root.create_dir("subdir").await?;

    // List directory contents
    for (name, entity) in root.get_entries() {
        println!("- {}: {:?}", name, entity);
    }

    // Persist changes; creates a new version of the directory
    let root_cid = root.checkpoint().await?;
    println!("Checkpoint root directory with CID: {}", root_cid);

    Ok(())
}
```

#### API Overview

- `File`: Represents a file in the filesystem
- `Dir`: Represents a directory in the filesystem
- `FileInputStream`: Provides read access to file contents
- `FileOutputStream`: Provides write access to file contents
- `Metadata`: Stores metadata for files and directories
- `Storable`: Trait for storing and loading entities from the content-addressed store

For more detailed examples and API usage, check out the `examples` directory and the API documentation.

## 💻 Development

To set up `monofs` for development:

1. Ensure you have Rust installed (latest stable version)
2. Clone the monocore repository:
   ```sh
   git clone https://github.com/appcypher/monocore
   cd monocore/monofs
   ```
3. Build the project:
   ```sh
   cargo build
   ```
4. Run tests:
   ```sh
   cargo test
   ```

## ⚖️ License

This project is licensed under the [Apache License 2.0](../LICENSE).
