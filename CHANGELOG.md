
## v0.4.0 - 2026-07-02







### :rocket: New features

- **(print_order)** Print the list of tasks in order defined in the file

- **(common)** `grim` now uses common terms like `tasks`

- Add `lang` command, CLI documentation, and bump version to 0.4.0

- **(script_files)** `grimoire` now supports running script files






### :bug: Bug fixes

- **(inambiguous)** Fixed inambiguous task resolution

- Use structured arguments for direct file execution

- Restore standard shell context for default language fallback



















## v0.3.0 - 2026-07-01







### :rocket: New features

- **(silent)** Add `silent = true` to suppress execution echo

- **(global_vars)** Global Variables support






### :bug: Bug fixes

- **(executor)** Correct variable shadowing and polish documentation



















## v0.2.0 - 2026-06-30







### :rocket: New features

- **(flag)** `grimoire` now supports CLI flags






### :bug: Bug fixes

- **(exe)** Remove executable file too

- **(injection)** Prevent shell injection in C/C++ sigils by splitting compile and run

- **(executor)** Preserve C/C++ exit codes by shifting cleanup to Rust

- **(executor)** Correct cross-language CLI arguments and refactor binary structure



















## v0.1.0 - 2026-06-29







### :rocket: New features

- **(C/C++ support)** Support for running c/cpp scripts

- **(init)** Init command added

- **(cli)** Cli commands added using `clap`

- **(list)** Lists all sigils






### :bug: Bug fixes

- **(language)** Ignoring language bug fixed



















### :tada: New Contributors
- @Vaishnav-Sabari-Girish made their first contribution
