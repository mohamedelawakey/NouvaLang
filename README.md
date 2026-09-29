# NouvaLang 🚀

NouvaLang is a modern, statically typed, open-source programming language designed to bring the best of both worlds: high performance and an elegant, developer-friendly syntax. Our vision is to create a high-performance language with first-class support for Artificial Intelligence and Machine Learning workloads natively.

---

## ✨ Highlights of NouvaLang (v0.1.0 MVP)

- **📦 Zero Prerequisites for Users:** Download standalone pre-compiled binaries for Linux, Windows, or macOS and start coding immediately. No compilers or runtimes required!
- **⚡ Blazing Fast Execution:** Native tree-walking AST evaluator with strict type checking.
- **🐍 Clean Syntax:** Intuitive, modern syntax with braces for clear block structure.
- **🛡️ Strict Static-like Typing:** Compile/runtime type enforcement for `int`, `float`, `bool`, and `str`.
- **🔄 Rich Control Flow & Recursion:** `if/else`, `while`, `for ... in ... to`, recursive function declarations, and nested scopes.
- **💻 CLI & Interactive REPL:** Dedicated `nouva` terminal command with interactive shell and file runner (`.nv`).

---

## 📦 Quick Installation (No Rust / No Dependencies Required)

You do **NOT** need Rust or any other runtime installed on your machine. Simply download the standalone pre-compiled binary for your operating system:

| Platform | Architecture | Archive Package |
| :--- | :--- | :--- |
| **Linux** | x86_64 | [Download `nouva-linux-x86_64.tar.gz`](https://github.com/mohamedelawakey/NouvaLang/releases/latest) |
| **Windows** | x86_64 | [Download `nouva-windows-x86_64.zip`](https://github.com/mohamedelawakey/NouvaLang/releases/latest) |
| **macOS** | x86_64 | [Download `nouva-macos-x86_64.tar.gz`](https://github.com/mohamedelawakey/NouvaLang/releases/latest) |

### Linux & macOS Quick Setup
```bash
# 1. Extract the downloaded archive
tar -xzvf nouva-linux-x86_64.tar.gz

# 2. Grant execution permission
chmod +x nouva

# 3. (Optional) Move to system PATH to use 'nouva' anywhere
sudo mv nouva /usr/local/bin/
```

### Windows Quick Setup
1. Extract `nouva-windows-x86_64.zip`.
2. Open PowerShell or CMD in the extracted folder and run:
```powershell
.\nouva.exe
```

---

## 🚀 How to Use

### 1. Interactive REPL (Shell)
Run `nouva` without arguments to start the interactive shell:
```bash
nouva
```
```text
NouvaLang REPL v0.1.0 (Interactive Shell)
Type 'exit' or press Ctrl+C to quit.

nouva> let age: int = 20;
nouva> let status: str = (age >= 18) ? "Adult" : "Minor";
nouva> print("Status:", status);
Status: Adult
null
nouva> exit
```

### 2. Running a Nouva Script File (`.nv`)
Create a file named `main.nv`:
```kotlin
// main.nv - Fibonacci Demo
fun fibonacci(n: int) -> int {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

let count: int = 8;
print("Fibonacci of", count, "is:", fibonacci(count));
```

Run it directly using the CLI:
```bash
nouva main.nv
# or using the run subcommand:
nouva run main.nv
```

---

## 🛠️ Building From Source (For Contributors Only)

If you are a contributor and wish to modify or build NouvaLang from source:

```bash
git clone https://github.com/mohamedelawakey/NouvaLang.git
cd NouvaLang
cargo build --release
```

Run unit tests (21 unit tests):
```bash
cargo test
```

---

## 🗺️ Project Architecture & Documentation

- **[NouvaLang Documentation](https://mohamedelawakey.github.io/NouvaLang-Docs/):** Complete developer guide and language tutorials.
- **[PLAN.md](PLAN.md):** Project Roadmap and completed architectural milestones.

---

## 🤝 Contributing
We welcome contributions from the community! Check out [CONTRIBUTING.md](CONTRIBUTING.md).

---

## 📜 License
> [!IMPORTANT]
> This project uses a **Custom License**.
> - The **Compiler Source Code** is strictly **NON-COMMERCIAL**.
> - **Programs built using NouvaLang** can be used/sold commercially with attribution (e.g., *"Built using NouvaLang"*).
>
> Please read the full [LICENSE](LICENSE) file for complete terms.
