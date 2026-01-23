# Synapse Knowledge Manager

> **Connect your thoughts** - A local-first knowledge management system combining the best of Notion and Obsidian

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)
[![Rust Edition](https://img.shields.io/badge/edition-2024-blue.svg)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)

## 🎯 Overview

Synapse Knowledge Manager (Synapse) is a powerful, local-first knowledge management and task management system designed for individuals who need efficient ways to organize, connect, and manage their thoughts.

### Core Features

- 🏠 **Local-First**: Your data stays on your device, ensuring privacy and offline access
- ⌨️ **Vim Keybindings**: Powerful editing experience with Vim shortcuts
- 🔗 **Rich Content Interactions**: Bidirectional links, database relations, block references
- 🔌 **Highly Customizable**: Plugin system for extensibility
- 📱 **Multi-Device Sync**: Optional sync support across devices
- 🤖 **AI Integration**: Optional RAG-powered AI assistance

## 🚀 Status

**Current Stage**: MVP Development (Phase 1)

This project is currently in early development. The MVP is being built with the following core features:

- [ ] Local storage (SQLite)
- [ ] Markdown editor (CodeMirror 6 + Vim mode)
- [ ] Note management (CRUD)
- [ ] Basic search
- [ ] Basic UI

See the project repository for the latest status and progress updates.

## 🏗️ Architecture

Synapse follows a **local-first architecture** with the following layers:

- **UI Layer**: Tauri (desktop) / Swift/Kotlin (mobile)
- **Core Logic**: Rust-based business logic and editor engine
- **Storage Layer**: SQLite + file system
- **Sync Layer** (optional): CRDT-based synchronization using Loro
- **Plugin System** (optional): Wasm-based plugin runtime
- **AI Service** (optional): RAG-powered AI features

For detailed architecture information, please refer to the project documentation (available separately).

## 🛠️ Tech Stack

- **Language**: Rust
- **CRDT**: Loro
- **Editor**: CodeMirror 6
- **Database**: SQLite
- **Desktop Framework**: Tauri
- **Sync Protocol**: WebSocket + custom protocol
- **Encryption**: AES-256-GCM + Argon2
- **Plugin System**: Wasm (wasmtime)

## 📦 Installation

> ⚠️ **Note**: Synapse is not yet available for installation. This section will be updated once the MVP is ready.

## 🚧 Development

### Prerequisites

- Rust 1.85 or later (Rust 2024 edition)
- Node.js 18+ (for Tauri, when UI is implemented)
- SQLite 3

### Building

```bash
# Clone the repository
git clone https://github.com/duskgrow/synapse-knowledge-manager.git
cd synapse-knowledge-manager

# Build the project
cargo build

# Run tests
cargo test

# Run the application
cargo run
```

### Project Structure

```
synapse-knowledge-manager/
├── src/
│   ├── main.rs        # Application entry point
│   ├── lib.rs         # Library entry point
│   ├── core/          # Core business logic
│   │   ├── mod.rs
│   │   ├── error.rs   # Error types
│   │   ├── models.rs  # Data models
│   │   └── services.rs # Service layer
│   ├── editor/        # Editor engine (CodeMirror 6 integration)
│   │   └── mod.rs
│   └── storage/       # Storage layer (SQLite + file system)
│       └── mod.rs
├── Cargo.toml         # Rust project configuration
├── LICENSE-MIT        # MIT License
├── LICENSE-APACHE     # Apache 2.0 License
└── README.md           # This file
```

> **Note**: This repository contains only the source code. Internal design documentation and project planning documents are maintained separately and are not included in this public repository.

## 🤝 Contributing

Contributions are welcome! However, please note that this project is in early development. 

1. Check the open issues to see what's planned
2. Open an issue to discuss major changes
3. Fork the repository and create a feature branch
4. Submit a pull request

### Development Guidelines

- Follow Rust naming conventions and style guidelines
- Add tests for new features
- Update documentation as needed
- Ensure all tests pass before submitting a PR

## 📄 License

This project is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## 🙏 Acknowledgments

Synapse is inspired by:
- [Notion](https://www.notion.so/) - For its powerful database and block-based editing
- [Obsidian](https://obsidian.md/) - For its local-first approach and bidirectional linking
- [Roam Research](https://roamresearch.com/) - For its knowledge graph and block references

## 📞 Contact

- **Project**: [Synapse Knowledge Manager](https://github.com/duskgrow/synapse-knowledge-manager)
- **Issues**: [GitHub Issues](https://github.com/duskgrow/synapse-knowledge-manager/issues)

---

**Note**: This project is under active development. Features and APIs may change without notice.
