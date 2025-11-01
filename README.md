# foxdir

> A fast, colorful, and lightweight Rust CLI tool to inspect directories with elegance — view file details in a beautiful table or export clean JSON output.

![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)
![CLI](https://img.shields.io/badge/CLI-Tool-yellow?style=for-the-badge)
![Cross Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-green?style=for-the-badge)

---

## About

**foxdir** is a lightweight, high-performance command-line utility built in Rust for exploring directories with style.

It displays file names, types, sizes, and modification times in a neatly formatted, colorized table or exports the same data as structured JSON for programmatic use.  
Designed for developers who want a clean, efficient, and visually appealing CLI experience.

---

## Features

- **Fast and efficient** — powered by Rust for optimal performance  
- **Colorized table view** using `owo-colors` and `tabled`  
- **JSON export** for easy integration or automation  
- **Local modification time** displayed using `chrono`  
- Works seamlessly with both files and directories  
- **Simple and intuitive CLI interface** powered by `clap`

---

## Installation

### Using Cargo

To install **foxdir** via Cargo, run the following command:

```sh
cargo install --path .
```

Or, install directly from GitHub:

```sh
cargo install --git https://github.com/Asura0666/foxdir.git
```

### Manual Build

1. Clone the repository:

```sh
git clone https://github.com/Asura0666/foxdir.git
```

2. Navigate into the project folder:

```sh
cd foxdir
```

3. Build the project in release mode:

```sh
cargo build --release
```

4. Run the binary from `target/release/foxdir`.

---

## Usage

### Default (Table View)

```sh
foxdir <path>
```

Example:

```sh
foxdir D:\Rust\Projects
```

### JSON Output

To export data in JSON format:

```sh
foxdir <path> --json
```

Or use the shorthand flag:

```sh
foxdir -j <path>
```

---

## Example Output

### Table View

```
selected path: D:\Rust\Projects\demo_cli
┌─────────────────────────────┬────────┬──────────────┬───────────────────────┐
│ Name                        │ Type   │ Bytes        │ Modified              │
├─────────────────────────────┼────────┼──────────────┼───────────────────────┤
│ main.rs                     │ File   │ 2048         │ 2025-10-31 14:22:08   │
│ src                         │ Dir    │ 0            │ 2025-10-30 19:15:10   │
│ Cargo.toml                  │ File   │ 220          │ 2025-10-29 22:10:02   │
└─────────────────────────────┴────────┴──────────────┴───────────────────────┘
```

### JSON Output

```json
[
  {
    "name": "main.rs",
    "e_type": "File",
    "len_bytes": 2048,
    "modified": "2025-10-31 14:22:08"
  },
  {
    "name": "src",
    "e_type": "Dir",
    "len_bytes": 0,
    "modified": "2025-10-30 19:15:10"
  }
]
```

---

## Command Options

| Flag           | Description                                                         |
| -------------- | ------------------------------------------------------------------- |
| `--json`, `-j` | Output directory details in JSON format                             |
| `<path>`       | The target directory path (defaults to current working directory)   |

---

## Tech Stack

* **Rust** — Core language
* **Clap** — Command-line argument parsing
* **Owo-Colors** — Terminal color styling
* **Tabled** — Table rendering
* **Chrono** — Date and time formatting
* **Serde / Serde JSON** — Serialization

---

## License

This project is licensed under the **MIT License**.  
See the [LICENSE](LICENSE) file for more information.

---

## Author

Name: **Dhiraj Lande**  
Email: [landedhiraj928@gmail.com](mailto:landedhiraj928@gmail.com)
