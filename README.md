# Grimoire

> A language-agnostic task runner where commands become sigils and automation
> becomes magic.

Inspired by the spellbooks of **Frieren** and **Witch Hat Atelier**, Grimoire is
a cross-platform automation tool for defining, documenting, and executing
reusable workflows.

I am using `grimoire` in this project too. Take a look at
[Grimoire.toml](./Grimoire.toml)

## Features

* Declarative `Grimoire.toml` configuration
* Tasks called **Sigils**
* Multi-language support (Shell, Python, Node, Bash, C, C++)
* **Native script file execution with cross-platform shebang auto-detection**
* Dependency management (DAG with cycle detection)
* **Interactive prompts and arguments via Terminal UI**
* **Silent execution modes**
* Cross-platform (Linux, macOS, Windows)

## Installation

### Via Cargo

```bash
cargo install grim
```

To install pre-compiled binaries

```bash
cargo binstall grim
```

For the latest development version:

```bash
cargo install --git https://github.com/Vaishnav-Sabari-Girish/grimoire
```

*(Note: Installing via cargo will provide both the `grimoire` and `grim`
executables).*

### From source

```bash
git clone https://github.com/Vaishnav-Sabari-Girish/grimoire.git
cd grimoire
cargo run --bin grim --release
```

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

List supported magical tongues (languages):

```bash
grim lang
# or use the alias: grim tongues
```

## Global Ingredients (Variables)

You can define global variables at the top of your `Grimoire.toml` using the
`[ingredients]` table to share values across multiple sigils.

*(Note: Local interactive arguments will automatically shadow and override
global ingredients if they share the same name).*

```toml
version = "1"

[ingredients]
target_dir = "./release_bins"
c_compiler = "gcc"

[sigil.build_c]
language = "c"
run = '''
{{c_compiler}} main.c -o {{target_dir}}/app
'''
```

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

## Silent Execution

By default, Grimoire echoes the exact command string it is about to run. You can
suppress this terminal output for specific tasks by adding `silent = true`.

```toml
[sigil.global]
description = "Prints global vars cleanly"
language = "shell"
silent = true
run = '''
echo {{hello_g}} 
echo {{bye_g}}
'''
```

## Passing Flags to Interactive Commands

Sometimes you want to pass extra command-line flags (like `--simulate` or
`--verbose`) directly to the underlying tool without the interactive menu
intercepting them.

**Option 1: Baked into Grimoire.toml (Recommended)** The most frictionless way
is to bake the standard `--` separator directly into your `run` string. This
tells underlying tools (like Cargo) to accept the appended flags naturally:

```toml
[sigil.run]
description = "Run the project"
language = "shell"
# Note the trailing '--' here so appended arguments go straight to the binary
run = "cargo run --features {{features}} --"
```

Now you can simply run `grim cast run --simulate` without any parsing errors!

**Option 2: In the CLI (Double Separator)** If you do not want to change your
`Grimoire.toml`, you must pass the separator in the CLI. Because Grimoire's own
CLI parser consumes the first `--` it sees, you need to use two if you want to
pass one down to the underlying command:

```bash
grim cast run -- -- --simulate
```

## Multi-Language Sigils

Grimoire does not assume everything is a shell script. Every sigil can specify
its own runtime. *(Note: The host machine must have the respective interpreter
or compiler installed).*

### Direct Script Execution (Shebangs)

If you are executing a physical file, Grimoire can natively read the file's
shebang (e.g., `#!/usr/bin/env python3`) to auto-detect the language! You can
omit the `language` field entirely.

Make sure your script files have execution rights (`chmod +x`), and wrap your
injected variables in single quotes:

```toml
[sigil.py]
description = "Cast a Python script directly"
silent = true
run = "scripts/test.py '{{target}}'"

[sigil.js]
description = "Cast a Node.js script directly"
silent = true
run = "scripts/test.js '{{target}}'"
```

### Interpreted Languages (Inline)

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

## Concepts

| Term | Meaning |
| --- | --- |
| Grimoire | Project configuration |
| Sigil | A task or command |
| Cast | Execute a sigil |
| Ingredients | Parameters and variables |

## Philosophy

Grimoire aims to make automation:

* Discoverable
* Self-documenting
* Portable
* Language-agnostic
* Pleasant to use

Underneath the magical terminology is a serious, production-grade automation
tool.

## Roadmap

* [x] Core task runner
* [x] Dependency engine (DAG)
* [x] Multi-language runtimes
* [x] Interactive prompts
* [x] Native shebang support
* [ ] Full TUI interface (`grim tui`)
* [ ] Graph visualization
* [ ] Project templates

## Inspiration

### Tooling

* `just`
* `make`
* `task`
* `cargo xtask`

### Theme

* **Frieren: Beyond Journey's End**
* **Witch Hat Atelier**

### License

MIT OR Apache-2.0

## 🧠 (mostly) Brain made

**This project was NOT vibe-coded BUT AI is still involved in some parts of
it.**

* **Generating test code:** Because it's something I always skip so I would
rather have some AI generated tests than none at all.
* **Micro-improvements:** I have used AI as an advisor to improve some bits of
code here and there. Big refactors or new features are done by my hand though.

<br>

![img](https://brainmade.org/white-logo.svg)
