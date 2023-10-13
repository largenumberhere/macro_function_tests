extern crate proc_macro;

use std::fmt::{Debug, Write};

use std::ops::Deref;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use quote::__private::ext::RepToTokensExt;
use syn::parse::{Parse, ParseStream};
use syn::{Error, File, FnArg, ItemFn, parse, parse_macro_input, Pat, ReturnType, Token, Type};
use syn::spanned::Spanned;

type TokenStream1 = proc_macro::TokenStream;
type TokenStream2 = proc_macro2::token_stream::TokenStream;

type TokenTree1 = proc_macro::TokenTree;
type TokenTree2 = proc_macro2::TokenTree;

// #[proc_macro]
// pub fn hello_fn_proc_macro(input: TokenStream1) -> TokenStream1 {
//     let output: TokenStream2 = quote!{
//         {
//             println!("Hello world!");
//             "Hello world"
//         }
//     };
//
//
//     let output: TokenStream1 = output.into();
//     output
// }

#[derive(Debug)]
struct AttributeInput{
    debug_mode: bool
}
impl Parse for AttributeInput{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty(){
            return  Ok(AttributeInput{
                debug_mode: false
            })
        }

        let input_identifier: Ident = input.parse()?;
        if !(input_identifier.to_string() == "Debug") {
            return Err(input.error("The only supported argument is 'Debug'"));
        }

        if !input.is_empty(){
            return Err(input.error("Expected only one argument to this attribute"));
        }

        return Ok(AttributeInput{
            debug_mode: true
        })
    }
}

#[proc_macro_attribute]
pub fn new_macro_function(attr: TokenStream1, items: TokenStream1) -> TokenStream1 {
    // expected Attribute input data variants:
    // #[macro_function] => TokenStream []
    //  #[macro_function(Debug)] => TokenStream [Ident{  ident: "Debug", span: ... }]
    let attr = parse_macro_input!(attr as AttributeInput);

    // TODO: In debug mode, try to point out syntax errors before giving output to compiler
    if attr.debug_mode {
        print_coloured(format_args!("warning: "), term::color::BRIGHT_YELLOW);
        println!("debug mode is enabled for this function:");
        let input_method_dbg = format_stream_n(&items);
        println!("Parsing method:");
        println!("'''\n{}\n'''", input_method_dbg);
    }

    let input_function = parse_macro_input!(items as ItemFn);

    let has_no_inputs = func_has_no_input(&input_function);
    let has_no_outputs = func_has_no_output(&input_function);

    let mut out;
    // Handle easy case
    if has_no_inputs && has_no_outputs {
        let func_name = &input_function.sig.ident;

        out = quote!(
            use ::proc_macro as _; //make sure it's in scope

            #[proc_macro]
            pub fn #func_name (___macro_function_macro_input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
                #input_function

                #func_name ();
                ::proc_macro::TokenStream::new()
            }
        );
    }
    else {

        let inputs = get_inputs(&input_function);
        let outputs = split_outputs(get_all_output(&input_function));

        let func_name = &input_function.sig.ident;
        let mut counter = 0..;
        let inputs_count = inputs.len();
        let input_var_iter = std::iter::from_fn(move || {
            let v = counter.next().expect("counter should be infinite...");
            if v == inputs_count {
                return None;
            }

            let ident: Ident = syn::parse_str(&format!("__input_var_{}", v)).expect("failed to parse ident for input_var_iter");
            return Some(ident);
        });
        let mut input_var_iter1 = input_var_iter.clone();
        let mut input_var_iter2 = input_var_iter.clone();
        let mut  input_var_iter3 = input_var_iter.clone();
        let mut input_var_iter4 = input_var_iter.clone();

        eprintln!("inputs: {:?}, count: {} ", inputs, inputs_count);

        out = quote!{
            use ::proc_macro as _; //make sure it's in scope

            #[proc_macro]
            pub fn #func_name (___macro_function_macro_input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
                // Decalare function
                # input_function

                // Work out all inputs

                // Create a parser for the inputs
                struct Inputs{
                    #(
                        #input_var_iter1 : #inputs
                    )*
                }

                impl ::macro_function::macro_items::_syn::parse::Parse for Inputs {
                    fn parse(input: ::macro_function::macro_items::_syn::parse::ParseStream) -> ::macro_function::macro_items::_syn::Result<Self> {
                        #(
                            let #input_var_iter2 = input.parse()?;
                        )*

                        let struct_ = Self {
                            #(
                                #input_var_iter3
                            ),*
                        };

                        return Ok( struct_ );
                    }
                }

                // Parse the inputs
                let ___inputs =  ::macro_function::macro_items::_syn::parse_macro_input!(___macro_function_macro_input as Inputs);

                // Call it with all the args
                let output = (
                    # func_name (
                        #(
                            ___inputs. #input_var_iter4
                        ),*
                ) );

                // TODO: Unpack out all outputs
                ::proc_macro::TokenStream::new()
            }



        };



        // // println!("{:?}", input_function.sig.inputs);
        // // println!("{:?}", input_function.sig.output);
        // todo!("Unandhled function");
        // out = TokenStream2::new()
    }

    if attr.debug_mode {
        let out_dbg = format_stream_n(&out);
        println!();
        println!("Parsed method:\n'''\n{}'''", out_dbg);
    }

    return out.into();

}

/// If output is a tuple, extract each item
fn split_outputs(output_value :&Box<syn::Type>) -> Vec<Box<syn::Type>> {
    let tup: Result<syn::TypeTuple, _> = syn::parse2(output_value.into_token_stream());
    match tup {
        Err(_) => {
            return vec![output_value.clone()];

        }
        Ok(tuple) => {
            let mut tuple_items = vec![];

            for item in tuple.elems {
                let item_boxed = Box::new(item);
                tuple_items.push(item_boxed);
            }

            return tuple_items;

        }
    }



}


// fn println_stream2(stream: &TokenStream2) {
//     let term = term::stdout().unwrap();
//
//
//     let file = syn::parse_file(&stream.to_string()).unwrap();
//     let output = prettyplease::unparse(&file);
//
//     println!("{}", output);
// }

// fn format_stream2(stream: &TokenStream2) -> String {
//     let file = syn::parse_file(&stream.to_string()).unwrap();
//     let output = prettyplease::unparse(&file);
//     output
// }
//
// fn format_stream1(stream: &TokenStream1) -> String {
//     let file = syn::parse_file(&stream.to_string()).unwrap();
//     let output = prettyplease::unparse(&file);
//     output
// }

trait TokenStreamN {}
impl TokenStreamN for TokenStream1 {}
impl TokenStreamN for TokenStream2 {}

fn format_stream_n<S>(stream: &S) -> String
    where S: std::fmt::Display + TokenStreamN + Debug
{
    let file: Result<syn::File, _> = syn::parse_file(&stream.to_string());
    let output;
    match file {
        Err(e) => {
            eprint!("Stream parseing error: Failed to parse stream as file {}", stream);
            output = stream.to_string();
        }
        Ok(v) => {
            output = prettyplease::unparse(&v);
        }
    }

    output
}

fn print_coloured(args: core::fmt::Arguments, colour: term::color::Color) {
    let mut term_handle = term::stdout().expect("Failed to grab terminal stdout");
    match term_handle.fg(colour) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to change terminal colour. Ignoring...\n{:#?}", e)
        }
    }
    term_handle.write_fmt(args).expect("Failed to format string for terminal output");
    match term_handle.reset() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to reset terminal. Colour may be mangled :(. Ignoring error...\n {:#?}", e);
        }
    };
    match term_handle.flush() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to flush terminal. Ignoring...");
        }
    };
}




fn func_has_no_input(func: &ItemFn) -> bool {
    if func.sig.inputs.len() == 0 {
        return true;
    }


    return false;
    //todo!("Unhandled function input case");
}


fn get_inputs(func: &ItemFn) -> Vec<&Box<syn::Type>> {
    let inputs = &func.sig.inputs;
    if inputs.len() == 0 {
        return Vec::new();
    }

    let mut inputs_list = Vec::with_capacity(inputs.iter().count());
    for input in inputs{
        let arg = match input {
            FnArg::Receiver(_) => {
                panic!("self is not yet supported for inputs")
            }
            FnArg::Typed(v) => {v}
        };

        let type_ = &arg.ty;
        // if arg.attrs.len() == 0 {
        //     panic!("attributes in inputs are unsupported");
        // }

        inputs_list.push(type_);
    }

    return inputs_list;
}

fn get_all_output(func: &ItemFn) -> &Box<syn::Type> {
    let outputs_raw = &func.sig.output;
    let output_type = match outputs_raw {
        ReturnType::Default => {
            panic!("Default return type not expected for output");
        }
        ReturnType::Type(_arrow,type_) => {
            type_
        }
    };

    return &output_type;
}

fn func_has_no_output(func: &ItemFn) -> bool {
    //println!("signature: {:#?}", func.sig);
    match &func.sig.output {
        ReturnType::Default => {
            return  true;
        }
        ReturnType::Type(_arrow, type_) => {
            match type_.deref() {
                Type::Tuple(t) => {
                    match t.elems.len() {
                        0 => {
                            return true;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }


    return false;

    // println!("{:#?}", func.sig.output);
    // todo!("Unhandled function output type");
}