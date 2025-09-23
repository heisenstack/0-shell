# 🐚 0-shell

```text
   ___        _          _ _
  / _ \  ___ | |__   ___| | |
 | | | |/ _ \| '_ \ / _ \ | |
 | |_| | (_) | | | |  __/ | |
  \___/ \___/|_| |_|\___|_|_|
             0-shell
```

_A minimalist Unix-like shell written from scratch in Rust._

## ✨ Features

🔹 **Custom Tokenizer**

- Correctly handles spaces, quotes, and escaped characters.

🔹 **Interactive Command Loop**

- `$` prompt
- Multi-line input
- Graceful exit with `Ctrl+D`

🔹 **File System Utilities** (implemented with Rust’s standard library):

| Command | Description                 | Flags / Notes              |
| ------- | --------------------------- | -------------------------- |
| `echo`  | Prints arguments            | `-n` (no newline)          |
| `pwd`   | Shows current directory     | —                          |
| `cd`    | Changes directory           | `~` (home), `-` (previous) |
| `ls`    | Lists directory contents    | `-l`, `-a`, `-F`           |
| `cat`   | Concatenates & prints files | Reads from stdin           |
| `mkdir` | Creates directories         | —                          |
| `cp`    | Copies files & dirs         | —                          |
| `rm`    | Removes files               | `-r` (directory)           |
| `mv`    | Moves or renames            | —                          |

---

## ⚡ Getting Started

### 🔨 Build & Run

Clone the repo and start the shell:

```bash
git clone https://learn.zone01oujda.ma/git/mdinani/0-shell.git
cd 0-shell
cargo run
```

You’ll see:

```bash
$
```

Now you can run your commands 🚀

---

## 📚 Why 0-shell?

✅ Learn how shells work under the hood  
✅ Practice Rust systems programming  
✅ Explore tokenization, parsing, and process management

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).

---

### 💡 Contributing

PRs and suggestions are welcome! If you have ideas for new features, improvements, or documentation, feel free to open an issue.
