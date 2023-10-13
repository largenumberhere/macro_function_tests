# macro_function
#### Warnings: 
Currently in  pre-alpha.  Not for production use. Expect breaking changes regularly. The name might  even be changed yet...

## Rust macros are hard. Let's make them easier!  
### Usage:
1. Add `macro_function` to your  dependencies  
(In pre-alpha you will need to add it manually like so under `[dependencies]`. Cargo will search the root repository for the `macro_function` project)
    ```
    macro_function = {git = "https://github.com/largenumberhere/macro_function_tests.git"}
    ```
2. slap the `#[macro_function::new_macro_function]` attribute on top of a normal function to transform it into a procedural macro.
