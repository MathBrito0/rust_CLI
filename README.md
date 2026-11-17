<div align="center">
  <img src="/assets/rust_cli.png" alt="Banner for the Unix File System Simulator in Rust" width="100%">
</div>

---

[![Rust](https://img.shields.io/badge/Rust-2021_Edition-ED3311?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Cargo](https://img.shields.io/badge/Build-Cargo-F05032?style=for-the-badge&logo=rust&logoColor=white)](https://doc.rust-lang.org/cargo/)
[![Status](https://img.shields.io/badge/Status-Completed-2ea44f?style=for-the-badge)]()

[About the Project](#-about-the-project) • [Architecture](#-architecture-and-modules) • [How to Run](#-how-to-run) • [Technical Journal](./docs/README.md)

</div>

---

## 📖 About the Project

This project is a command-line application developed in **Rust** that simulates the logical behavior of a file system and access control, modeled after **Unix/Linux** operating systems.

The primary focus of the development was to apply fundamental Operating System concepts—such as `UID`/`GID` identification, directory hierarchy, and the calculation of octal permissions (`rwx`)—while leveraging the memory safety guarantees (*ownership* and *borrowing*) provided by the Rust programming language.

> [!NOTE]
> **Note on repository organization:** This repository maintains the **original modular architecture**, with code organized into separate folders and files to facilitate readability and maintenance. For details on how the project was adapted into a single file (`main.rs`) for academic submission, as well as the challenges encountered with the compiler, please refer to our **[Technical Journal](./docs/README.md)**. ---

## 🏗️ Architecture and Modules

The system is designed by separating business logic (`models`) from the user interaction interface (`menus`):

```text
src/
├── main.rs                   # Entry point and in-memory vector initialization
├── models/                   # Entities and business logic (Structs & Impls)
│   ├── users.rs              # User management and group associations
│   ├── groups.rs             # Group management and member lists
│   ├── directories.rs        # Hierarchical container for storing files
│   ├── archives.rs           # Metadata, size, owner, and `stat`-style display
│   └── permissions.rs        # Boolean logic (r, w, x) and octal conversion
└── menus/                    # Terminal-based interaction layer (CLI)
├── main_menu.rs              # Central router for submenus
├── users_menu.rs             # User CRUD operations and input helpers
├── group_menu.rs             # Group CRUD operations and associations
├── dir_menu.rs               # Directory creation and editing
└── archives_menu.rs          # File creation, permission modification, and status
```

### Summary of Entities

| Module | Main Structure | Technical Responsibility |
| :--- | :--- | :--- |
| **`Usuario`** | `uid: u16`, `grupos_ids: Vec<u16>` | Represents a system user and tracks the GIDs to which they belong. |
| **`Grupo`** | `gid: u16`, `membros_uids: Vec<u16>` | Groups users via their UIDs, enabling a bidirectional link. |
| **`Diretorio`** | `arquivos: Vec<Arquivo>` | Acts as a container for files, managing their addition, removal, and listing. |
| **`File`** | `permissions: (Permission, Permission, Permission)` | Stores metadata (name, size in bytes, UID, GID) and displays details via `stat()`. |
| **`Permission`** | `read`, `write`, `execute` (`bool`) | Calculates the octal value (e.g., `755`, `644`) for Owner, Group, and Others. |

---

## 💻 How to Run

### Prerequisites
* Have the **Rust** environment and the **Cargo** package manager installed ([Official installation guide](https://www.rust-lang.org/tools/install)).

### Step-by-Step
1. **Clone the repository:**
```bash
git clone [https://github.com/MathBrito0/rust_CLI.git](https://github.com/MathBrito0/rust_CLI.git)
cd your-repository
```

2. **Compile and run the program:**
```bash
cargo run
```

### Usage Example (Terminal Output)

```text
=== MAIN MENU ===
1. User menu.
2. Group menu.
3. Directory menu.
4. File menu.
0. Exit the program.
> 4

=== FILES ===
1. Create file
2. List files
3. View stats
4. Change permissions
5. Remove file
0. Back
> 3
File name: relatorio.txt
File: relatorio.txt
Size: 1024 bytes
Permissions: (664 / rw-rw-r--)
UID: 1000
GID: 100
```

---

## 🔮 Future Improvements (Roadmap)

- [ ] **Data Persistence:** Save and load the state of users, groups, and files in JSON or binary format (`serde`).
- [ ] **Nested Directories:** Allow directories to contain subdirectories recursively.
- [ ] **Access Validation:** Verify whether the logged-in `UID` has read/write permissions before modifying a file.

---

## 👥 Authors

| Developers | 
| :--- |
| [@MathBrito0](https://github.com/MathBrito0) |
| [@komodospit](https://github.com/komodospit) |
