# macro_function
Warnings: Currently in  pre-alpha.  Not for production use. Expect breaking changes regularly.

## Rust macros are hard. Let's make them easier!  
### Usage:
1. Add `macro_function` to your  dependencies  
(In pre-alpha you will need to add it manually as `macro_function = {git = ""}`, where `""` contains a .git url to the github root. **Do not use this sub-project**'s path. Cargo will search the root repository for the `macro_function` project)
2. slap the `#[macro_function::new_macro_function]` attribute on top of a normal function to transform it into a procedural macro.
