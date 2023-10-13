use macro_function::macro_items::{AnyTokens, LitStr, _syn::Ident};
use macro_function::macro_items::_syn::__private::ToTokens;
use macro_function::new_macro_function;
#[new_macro_function(Debug)]
fn my_macro() {
    println!("hello world!")
}

#[new_macro_function]
fn my_macro2() -> () {
    println!("Hello world again!")
}

#[new_macro_function(Debug)]
fn my_macro3() -> AnyTokens {
    AnyTokens::default()
}

#[new_macro_function(Debug)]
fn my_macro4() -> (LitStr, AnyTokens) {
    let str: LitStr = ::macro_function::macro_items::_syn::parse_str("\"hello world\"").unwrap();

    (str, AnyTokens::default())
}

#[new_macro_function(Debug)]
fn my_macro5(str: LitStr) -> () {
    println!("{:?}", str);
}

/// Print the text contained by a thingymabobber
#[new_macro_function(Debug)]
fn print_raw(idetntifier: Ident) -> () {
    println!("{}", idetntifier.to_string());
}

#[new_macro_function(Debug)]
fn spam_string(string: LitStr) -> AnyTokens {
    let mut stream = string.clone().to_token_stream();
    string.to_tokens(&mut stream);

    stream


}