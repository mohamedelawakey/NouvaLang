# NouvaLang 🚀

NouvaLang is a modern, statically typed, open-source programming language designed to bring the best of both worlds: the raw performance of **C** and the elegant, developer-friendly syntax of **Python**. Built entirely from scratch in Rust, our ultimate vision is to create a high-performance language with first-class support for Artificial Intelligence and Machine Learning workloads natively.

---

## ✨ Highlights of NouvaLang (v0.1.0 MVP)

- **⚡ Blazing Fast Execution:** Tree-walking AST evaluator with strict type checking.
- **🐍 Clean Syntax:** Intuitive, Python/Swift-like syntax with braces for block structure.
- **🛡️ Strict Static-like Typing:** Compile/runtime type enforcement for `int`, `float`, `bool`, and `str`.
- **🔄 Rich Control Flow & Recursion:** `if/else`, `while`, `for ... in ... to`, recursive function declarations, and nested scopes.
- **💻 CLI & Interactive REPL:** Dedicated `nouva` terminal command with interactive shell and file runner (`.nv`).
- **🦀 Built with Rust:** Maximum safety, zero memory leaks, and blazing-fast toolchain.

---

## 📦 Installation & Setup

### Prerequisites
Make sure you have [Rust & Cargo](https://rustup.rs/) installed:
```bash
rustc --version
cargo --version
```

### Install Nouva CLI Globally
Clone the repository and install the binary:
```bash
git clone https://github.com/mohamedelawakey/NouvaLang.git
cd NouvaLang
cargo install --path .
```

Now `nouva` is available globally in your terminal!

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
nouva> exit
```

### 2. Running a Nouva Script File (`.nv`)
Create a file named `main.nv`:
```swift
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

Run it using the CLI:
```bash
nouva run main.nv
# or directly:
nouva main.nv
```

---

## 🧪 Running Unit Tests

NouvaLang comes with a comprehensive test suite (21 unit tests) covering the Lexer, Pratt Parser, Evaluator, Scoping, and Type Checker:
```bash
cargo test
```

---

## 🗺️ Project Architecture & Documentation

- **[flow.md](flow.md):** Complete visual pipeline and architecture diagrams explaining how code transforms from raw text to execution.
- **[flow_code.md](flow_code.md):** Detailed module breakdown (`tokens`, `lexer`, `ast`, `parser`, `object`, `evaluator`, `repl`).
- **[language_v0_1_specification.md](language_v0_1_specification.md):** Formal v0.1.0 MVP Specification.

---

## 📋 Status (v0.1.0 MVP - ✅ COMPLETED)

- [x] **Phase 1: Lexical Analysis (Lexer)**
  - [x] Full keyword, operator, and literal tokenization
  - [x] Compound operators (`==`, `!=`, `>=`, `<=`, `+=`, `-=`, `*=`, `/=`, `%=`, `->`)
  - [x] Escape characters in string literals (`\n`, `\t`, `\"`, `\\`)
  - [x] Single-line `//` and multi-line `/* */` comments
- [x] **Phase 2: Parsing (Parser)**
  - [x] Abstract Syntax Tree (AST) definitions
  - [x] Pratt Parser for mathematical and logical expressions with precedence
  - [x] Variable and constant statements (`let`, `const`)
  - [x] Function declarations (`fun`) and Function Calls (`print(...)`, `custom_func(...)`)
  - [x] Control flow (`if/else`) and Loops (`while`, `for i in start to end`)
  - [x] Ternary operator (`? :`) and Type Casting (`as`)
- [x] **Phase 3: Evaluation Engine (Runtime)**
  - [x] Object system and Scoped Memory Environment (Lexical Scoping)
  - [x] Strict Type Checking on assignment and function arguments
  - [x] Full recursion and return value propagation
  - [x] Built-in `print(...)` function
- [x] **Phase 4: CLI & REPL**
  - [x] Global `nouva` executable CLI
  - [x] File execution (`nouva run <file.nv>`)
  - [x] Interactive REPL prompt

---

## 🤝 Contributing
We welcome contributions from the community! If you are interested in compilers, programming language design, or future AI integrations, check out [CONTRIBUTING.md](CONTRIBUTING.md).

---

## 📜 License
> [!IMPORTANT]
> This project uses a **Custom License**.
> - The **Compiler Source Code** is strictly **NON-COMMERCIAL**.
> - **Programs built using NouvaLang** can be used/sold commercially with attribution (e.g., *"Built using NouvaLang"*).
>
> Please read the full [LICENSE](LICENSE) file for complete terms.
