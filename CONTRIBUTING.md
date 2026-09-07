# Contributing to NouvaLang

First off, thank you for considering contributing to NouvaLang! 🎉 It's people like you that make open-source such a great community.

> [!WARNING]
> Before contributing, please make sure you have read and understood our custom [LICENSE](LICENSE). By submitting a Pull Request, you agree that your contributions will be licensed under these exact terms.

## 🛠️ How Can I Contribute?

### 1. Reporting Bugs
If you find a bug in the lexer, parser, or any other component, please open an issue on GitHub. 
**Important:** Please ensure you add the appropriate label (e.g., `bug`) to your issue. Include:
- A clear description of the problem.
- The input code that caused the issue.
- The expected vs actual behavior.

### 2. Suggesting Enhancements
Have an idea for a new language feature or syntax? We are completely open to suggestions! Open an issue on GitHub, add the `enhancement` label, and describe your idea in detail.

### 3. Working on Open Issues
If you see an issue that you'd like to work on (especially those marked as `good first issue` or `help wanted`):
1. **Drop a comment** on the issue saying you'd like to work on it.
2. **Wait for a maintainer** to assign it to you so nobody else duplicates your work.
3. Once assigned, follow the Pull Request workflow below!

### 4. Submitting Pull Requests
If you want to dive into the code, here is the workflow to follow:

1. **Fork the Repository:** Create your own fork of the project on GitHub.
2. **Clone it locally:** `git clone https://github.com/your-username/NouvaLang.git`
3. **Create a Branch:** `git checkout -b feature/your-feature-name` or `bugfix/your-bugfix-name`
4. **Make your changes:** Ensure your code follows the existing style and conventions.
5. **Test your changes:** Make sure your new code doesn't break existing functionality.
6. **Commit your changes:** Write a clear and descriptive commit message.
7. **Push to the branch:** `git push origin feature/your-feature-name`
8. **Open a Pull Request (PR):** Submit the PR against the `main` branch of this repository.

## 🧪 Testing Requirements (Strict)
NouvaLang requires **rigorous and violent testing** for any new feature or bug fix. A PR will **NOT** be merged unless it includes comprehensive unit tests.
- **Cover Edge Cases:** Don't just test the "happy path". Test weird formatting, lack of spaces, and complex/nested scenarios.
- **Multiple Scenarios:** Write multiple assertions simulating heavy real-world usage (e.g., mixing operators, complex conditions, edge-case syntax).
- Look at `tests/unit/lexer_test.rs` for examples of the "violent testing" standard expected in this project.

## 🧑‍💻 Code Style
- Since the compiler is written in **Rust**, please ensure you use `cargo fmt` to format your code before submitting a PR.
- Add comments where necessary, especially for complex compiler logic.

## 💬 Communication
If you need help understanding the codebase or want to discuss a major change, feel free to open a Discussion on GitHub or reach out in the issues section.

For security vulnerabilities or direct inquiries, you can reach out to me directly at: mohamedelawakey@gmail.com

Let's build something awesome together! 🚀
