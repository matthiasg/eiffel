//! # Eiffel Inspired Invariant Checking
//!
//! This crate provides a set of macros inspired by the Eiffel programming language's
//! features for invariant checking. These features include checks for loops, entry, exit, and more.
//!
//! The Eiffel language's options for invariant checking serve as the basis for the design
//! and functionality of the macros in this crate.
//!
//! Please note that this crate is still a work in progress. As such, some features may not be fully
//! implemented or may undergo significant changes in future updates.
//!
//! Contributions and feedback are always welcome.
#![deny(warnings)]
#![deny(missing_docs)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{ quote, format_ident };
use syn::{parse_macro_input, ItemFn, ReturnType, FnArg, Pat, Ident, Meta};
use syn::punctuated::Punctuated;

enum CheckTime {
    Before,
    After,
    BeforeAndAfter,
}

/// `check_invariant` is a procedural macro that checks if a given invariant holds true before and after a method call.
/// If the invariant does not hold, the macro will cause the program to panic with a specified message.
/// 
/// # Arguments
/// 
/// * `invariant`: A method that returns a boolean. This is the invariant that needs to be checked.
/// 
/// # Example
///
/// ```
/// use eiffel_macros_gen::check_invariant;
/// 
/// struct MyClass {
///     // Fields
///     a: i32,
/// };
///
/// impl MyClass {
///     fn my_invariant(&self) -> bool {
///         // Your invariant checks here
///         true
///     }
///
///     #[check_invariant(my_invariant)]
///     fn my_method(&self) {
///         // Method body
///         println!("Method body {:?}", self.a);
///     }
/// }       
/// ```
///
/// # Test
///
/// ```
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///
///     #[test]
///     fn test_my_method() {
///         let my_class = MyClass;
///         my_class.my_method(); // This should not panic as the invariant is true
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn check_invariant(attr: TokenStream, item: TokenStream) -> TokenStream {
    let macro_args = parse_macro_input!(attr with Punctuated::<Meta, syn::Token![,]>::parse_terminated);
   
    let invariant_name = match macro_args.first() {
        Some(Meta::Path(path)) => path.get_ident().unwrap().clone(),
        _ => panic!("Expected the name of the function as the first argument"),
    };

    let mut check_time = CheckTime::BeforeAndAfter;

    for arg in macro_args.iter().skip(1) {
        match arg {
            Meta::NameValue(name_value) => {
                // Check if the argument is check_time == "before" or "after" or "before_and_after"
                if name_value.path.is_ident("check_time") {
                    if let syn::Expr::Lit(lit_str) = &name_value.value {
                        if let syn::Lit::Verbatim(value) = &lit_str.lit {
                            match value.to_string().as_str() {
                                "before" => check_time = CheckTime::Before,
                                "after" => check_time = CheckTime::After,
                                "before_and_after" => check_time = CheckTime::BeforeAndAfter,
                                _ => panic!("Invalid argument for check_time. Expected 'before', 'after', or 'before_and_after'")
                            }
                        }
                    } else {
                        panic!("Expected a string literal for the check_time argument");
                    }
                }
            },
            _ => ()
        }
    };

    // Extract the name, arguments, and return type of the input function
    let input_fn = parse_macro_input!(item as ItemFn);
    let input_fn_name = &input_fn.sig.ident;
    let input_fn_body = &input_fn.block;

    let args = &input_fn.sig.inputs;
    let arg_names: Vec<Ident> = args
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat) = arg {
                if let Pat::Ident(pat_ident) = &*pat.pat {
                    return Some(pat_ident.ident.clone());
                }
            }
            None
        })
        .collect();
    
    let _self_arg = match args.first() {
        Some(FnArg::Receiver(receiver)) => receiver,
        _ => panic!("The input function must have a self argument"),
    };

    let return_type = match &input_fn.sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => Some(quote! { #ty }),
    };

    // Rename the original function
    let fn_without_invariant = format_ident!("{}_no_invariant", input_fn_name);
    
    let wrapped_function = match &return_type {
        None => quote! {
            fn #fn_without_invariant(#args) { 
                #input_fn_body
            }
        },
        Some(return_type) => quote! {
            fn #fn_without_invariant(#args) -> #return_type { 
                #input_fn_body
            }
        }
    };

    let call_invariant_before = match check_time {
        CheckTime::Before | CheckTime::BeforeAndAfter => quote! {
            if !self.#invariant_name() {
                panic!("Invariant {} failed on entry", stringify!(#invariant_name));
            }
        },
        _ => quote! {},
    };

    let call_invariant_after = quote! {
        CheckTime::After | CheckTime::BeforeAndAfter => quote! {
            if !self.#invariant_name() {
                panic!("Invariant {} failed on exit", stringify!(#invariant_name));
            }
        },
        _ => quote! {},
    };

    let call_wrapped = quote! {
        self.#fn_without_invariant( #(#arg_names),*)
    };

    let invariant_checked_function = match return_type {
        None => quote! {
            fn #input_fn_name(#args) { 
                #call_invariant_before
                #call_wrapped;
                #call_invariant_after
            }
        },
        Some(return_type) => quote! {
            fn #input_fn_name(#args) -> #return_type {
                #call_invariant_before
                let result = #call_wrapped;
                #call_invariant_after
                result
            }
        }
    };

    // Generate the wrapper code
    let output = quote! {
        #wrapped_function
    
        #invariant_checked_function
    };

    output.into()
}
