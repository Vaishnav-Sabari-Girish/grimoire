# 🔮 Grimoire

> A language-agnostic task runner where commands become sigils and automation
> becomes magic.

Inspired by the spellbooks of **Frieren** and **Witch Hat Atelier**, Grimoire is
a cross-platform automation tool for defining, documenting, and executing
reusable workflows.

I am using `grimoire` in this project too. Take a look at
[Grimoire.toml](./Grimoire.toml)

## Features (Being Implemented)

* 📖 Declarative `Grimoire.toml` configuration
* 🪄 Tasks called **Sigils**
* 🌍 Multi-language support
  * Shell
  * Python
  * JavaScript / TypeScript
  * Lua
  * PowerShell
  * and more
* 🔗 Dependency management
* 📝 Self-documenting workflows
* 🎛 Interactive prompts and arguments
* 🌲 Dependency graph visualization
* 🔌 Extensible plugin system (**Familiars**)
* 💻 Cross-platform (Linux, macOS, Windows)

---

## Installation

> Installation instructions will be available once the first release is
> published.

```bash
cargo install --git https://github.com/Vaishnav-Sabari-Girish/grimoire
```

---

## Quick Start

Initialize a new project:

```bash
grimoire init
```

Create a `Grimoire.toml`:

```toml
version = "1"

[sigil.build]
description = "Compile the project"
run = "cargo build"

[sigil.test]
description = "Run tests"
run = "cargo test"

[sigil.release]
description = "Build release artifacts"
depends = ["build", "test"]
run = "cargo dist build"
```

`grimoire` uses different terminologies for commands, run etc. You can refer

## concepts for the proper terminologies

Run a sigil:

```bash
grimoire cast build
```

List available sigils:

```bash
grimoire sigils
```

Inspect a sigil:

```bash
grimoire inspect release
```

---

## Multi-Language Sigils

Python:

```toml
[sigil.hello]
language = "python"
run = """
print("Hello from Grimoire!")
"""
```

JavaScript:

```toml
[sigil.hello]
language = "javascript"
run = """
console.log("Hello from Grimoire!");
"""
```

Shell:

```toml
[sigil.hello]
language = "shell"
run = """
echo "Hello from Grimoire!"
"""
```

---

## Concepts

| Term        | Meaning                  |
| ----------- | ------------------------ |
| Grimoire    | Project configuration    |
| Sigil       | A task or command        |
| Cast        | Execute a sigil          |
| Ingredients | Parameters and variables |
| Familiars   | Plugins and extensions   |
| Codex       | Generated documentation  |

---

## Example

```bash
grimoire cast build
grimoire cast flash
grimoire cast release
```

Or using the short alias:

```bash
grim build
grim flash
```

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

* [ ] Core task runner
* [ ] Dependency engine
* [ ] Multi-language runtimes
* [ ] Interactive prompts
* [ ] Documentation generation
* [ ] TUI interface
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
