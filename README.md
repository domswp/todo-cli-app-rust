# todo-cli-app-rust
 simple command-line interface (CLI) todo list app built using Rust.   This project is created as a learning exercise to explore basic Rust syntax, `Vec`, input/output, and flow control using `match`.

---

## 🚀 Features

- Display todo list
- Add new todo items
- Remove todo items by number
- Looping interactive menu
- All handled via clean terminal interface

---

## 📦 Requirements

- Rust (recommended via `rustup`)
- Linux/macOS/Windows with terminal support

---

## 🛠️ Installation & Usage

### 1. Clone the Repository

```bash
git clone https://github.com/domswp/todo-cli-app-rust
cd todo-cli
```

### 2. Run the App

```bash
cargo run
```

---

## 💻 Example Screenshot

```bash
==== TODO LIST ====
1. Belajar Rust
2. Nonton video programming

Menu:
1. Tambah Todo
2. Hapus Todo
3. Keluar
Pilih opsi (1-3):
```
---

## ✍️ Code Highlights

   Built using Rust's standard I/O (`std::io`)

   Uses `Vec<String>` to store tasks

   Interactive `loop` and `match` control flow

   Input validation with `.parse::<usize>()`

---

## 🔮 Future Improvements

   Save & load todos from file (`.txt` or `.json`)

   Add categories or deadlines

   Use `struct` and modules for better structure

   Cross-platform compiled binary

---

## 📚 License

This project is open source and available under the MIT License.
   
