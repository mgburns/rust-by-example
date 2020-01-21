# 7. Managing growing projects with packages, crates, and modules

> As you write large programs, organizing your code will be important because keeping track of your entire program in your head will become impossible. By grouping related functionality and separting code with distinct features, you'll clarify where to find code that implements a particular feature and where to go to change how a feature works.

## Packages and Crates
- A crate is a binary or library. Groups related functionality in a scope for easy sharing.
- A package holds crates -- zero or one library crate and as many binary crates as you'd like
- Convention dictates that `src/main.rs` is root for binary crate, and `src/lib.js` is root for library crate, both named after the package. Additional binaries can be added in `src/bin`, one file per crate.

## Defining Module to Control Scope and Privacy
- Modules organize code within a crate for readability and reuse
- Control privacy

