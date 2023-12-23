extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{FnArg, Generics, ItemFn, parse_macro_input};
use syn::punctuated::Punctuated;
use syn::token::Comma;


#[proc_macro_attribute]
pub fn ai_function(_attr: TokenStream, item: TokenStream) -> TokenStream {

    // Parse the input function
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);

    // Create string representation of function
    let function_str: String = format!("{}", input_fn.to_token_stream());

    // Define a new function with the same signature as the input function
    let fn_ident: Ident = input_fn.sig.ident;
    let fn_inputs: Punctuated<FnArg, Comma> = input_fn.sig.inputs;
    let fn_generics: Generics = input_fn.sig.generics;

    // Generate output function
    let output = quote! {
        pub fn #fn_ident #fn_generics(#fn_inputs) -> &'static str {
            #function_str
        }
    };

    output.into()
}
