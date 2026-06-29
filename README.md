# 🔮 Grimoire

> A language-agnostic task runner where commands become sigils and automation
> becomes magic.

Inspired by the spellbooks of **Frieren** and **Witch Hat Atelier**, Grimoire is
a cross-platform automation tool for defining, documenting, and executing
reusable workflows.

I am using `grimoire` in this project too. Take a look at
[Grimoire.toml](./Grimoire.toml)

## Features

* 📖 Declarative `Grimoire.toml` configuration
* 🪄 Tasks called **Sigils**
* 🌍 Multi-language support (Shell, Python, Node, Bash, C, C++)
* 🔗 Dependency management (DAG with cycle detection)
* 🎛 **Interactive prompts and arguments via Terminal UI**
* 📝 Self-documenting workflows
* 💻 Cross-platform (Linux, macOS, Windows)

---

## Installation

**Via Cargo:**

I have not published it to [crates.io](https://crates.io/) yet.

```bash
cargo install --git https://github.com/Vaishnav-Sabari-Girish/grimoire

```

*(Note: Installing via cargo will provide both the `grimoire` and `grim`
executables).*

---

## Quick Start

Initialize a new project:

```bash
grim init

```

Create a `Grimoire.toml`:

```toml
version = "1"

[sigil.hello]
description = "A Simple Welcome spell"
language = "shell"
run = "echo 'Welcome to Grimoire!'"

# Testing default language fallback
[sigil.bye]
description = "Bye Spell"
run = "echo 'Bye From Grimoire'"

```

Run a sigil:

```bash
grim cast hello

```

List available sigils:

```bash
grim sigils

```

---

## Interactive Ingredients (Arguments)

Grimoire can pause execution and prompt the user for input using beautiful
terminal menus. Use the `{{variable}}` syntax to inject choices into your run
strings.

```toml
# Testing Options
[sigil.options]
description = "Try out option selection"
run = "echo 'Selection option is {{opt}}'"

[sigil.options.args.opt]
type = "select"
choices = [
  "Option 1",
  "Option 2",
  "Option 3"
]
default = "Option 1"

```

When you run `grim cast options`, Grimoire will open an interactive TUI menu for
you to select the desired option before executing the spell!

---

## Multi-Language Sigils

Grimoire does not assume everything is a shell script. Every sigil can specify
its own runtime. *(Note: The host machine must have the respective interpreter
or compiler installed).*

### Interpreted Languages

Python and JavaScript execute directly via their respective binaries:

```toml
[sigil.hello_py]
language = "python"
description = "Hello in Python"
run = "print('Hello World')"

[sigil.hello_js]
language = "javascript"
description = "Hello in javascript"
run = """
console.log("Hello World");
"""

```

### Compiled Languages (C / C++)

For C and C++, Grimoire acts as a seamless wrapper. It automatically scribes
your code to a temporary file, compiles it (via `gcc` or `g++`), executes the
resulting binary, and cleans up the temporary files—all in a fraction of a
second.

**Important:** Always use TOML literal strings (three single quotes `'''`) for
C/C++ to prevent TOML from escaping newline characters (`\n`).

```toml
[sigil.hello_c]
language = "c"
description = "Hello in C"
run = '''
#include <stdio.h>

int main() {
  printf("Hello from C\n");
  return 0;
}
'''

[sigil.hello_cpp]
language = "cpp"
description = "Hello in C++"
run = '''
#include <iostream>

int main() {
    std::cout << "Hello from C++!" << std::endl;
    return 0;
}
'''

```

---

## Concepts

| Term | Meaning |
| --- | --- |
| Grimoire | Project configuration |
| Sigil | A task or command |
| Cast | Execute a sigil |
| Ingredients | Parameters and variables |
| Familiars | Plugins and extensions |
| Codex | Generated documentation |

---

## Philosophy

Grimoire aims to make automation:

* Discoverable
* Self-documenting
* Portable
* Language-agnostic
* Pleasant to use

Underneath the magical terminology is a serious, production-grade automation
tool.

---

## Roadmap

* [x] Core task runner
* [x] Dependency engine (DAG)
* [x] Multi-language runtimes
* [x] Interactive prompts
* [ ] Documentation generation
* [ ] Full TUI interface (`grim tui`)
* [ ] Graph visualization
* [ ] Plugin system
* [ ] Project templates

---

## Inspiration

### Tooling

* `just`
* `make`
* `task`
* `cargo xtask`

### Theme

* **Frieren: Beyond Journey's End**
* **Witch Hat Atelier**

---

### License

MIT OR Apache-2.0
