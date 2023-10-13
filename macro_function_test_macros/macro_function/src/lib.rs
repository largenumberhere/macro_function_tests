// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

//pub use proc_macros::hello_fn_proc_macro;
pub use proc_macros::new_macro_function;
pub mod macro_items{
    pub mod _syn {
        pub use syn::*;
    }
    // /// A function definition
    // pub use syn::ItemFn;

    //A string in quotes in source code.
    pub use syn::LitStr;

    /// Several tokens of unspecified type
    pub use proc_macro2::TokenStream as AnyTokens;

}

