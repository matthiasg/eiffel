//! This create contains the procedural macros
//! 
//! Mostly the procedural macro [check_invariant](macro@check_invariant) which is used to check if a given invariant holds true before and after a method call.
//!
#![deny(warnings)]
#![deny(missing_docs)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{ quote, format_ident };
use syn::{parse_macro_input, ItemFn, ReturnType, Result, FnArg, Pat, Ident};
use syn::parse::{Parse, ParseStream};
use proc_macro2::TokenTree;
use syn::token::Comma;

enum CheckTime {
    #[allow(dead_code)]
    Before,
    #[allow(dead_code)]
    After,
    BeforeAndAfter,
}

struct AttrList {
    #[allow(dead_code)]
    invariant_function_identifier: Ident,
    #[allow(dead_code)]
    rest: Vec<TokenTree>,
}

impl Parse for AttrList {
    fn parse(input: ParseStream) -> Result<Self> {
        let first_ident: Ident = input.parse()?;

        if input.is_empty() {
            return Ok(AttrList { invariant_function_identifier: first_ident, rest: vec![] });
        }

        let mut rest = Vec::new();

        while !input.is_empty() {
            let _: Comma = input.parse()?;
            let item: TokenTree = input.parse()?;
            rest.push(item);
        }

        Ok(AttrList { invariant_function_identifier: first_ident, rest })
    }
}

/// `check_invariant` is a procedural macro that checks if a given invariant holds true before and after a method call.
/// If the invariant does not hold, the macro will cause the program to panic with a specified message.
/// 
/// # Arguments
/// 
/// * `invariant`: A method that returns a boolean. This is the invariant that needs to be checked.
/// * `check_time`: An optional string literal that specifies when the invariant should be checked.
///   * `"before"` - The invariant is checked before the operation.
///   * `"after"` - The invariant is checked after the operation.
///   * `"before_and_after"` - The invariant is checked both before and after the operation.
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
///
///     // Only check the invariant before the method call
///     #[check_invariant(my_invariant, "before")]
///     fn my_other_method(&self) {
///         // Method body
///         println!("Method body {:?}", self.a);
///     }
///
///     // Only check the invariant after the method call
///     #[check_invariant(my_invariant, "after")]
///     fn my_other_method_after(&self) {
///         // Method body
///         println!("Method body {:?}", self.a);
///     }
///
///     // Only check the invariant before and after (default)
///     #[check_invariant(my_invariant, "before_and_after")]
///     fn my_other_method_before_and_after(&self) {
///         // Method body
///         println!("Method body {:?}", self.a);
///     }
///
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
    // let invariant_name = parse_macro_input!(attr as Ident);
    // let check_time = CheckTime::BeforeAndAfter;
    let mut check_time = None;
    
    let attr = parse_macro_input!(attr as AttrList);
    let invariant_name = attr.invariant_function_identifier;

    for item in attr.rest.into_iter() {
        match item {
            TokenTree::Literal(literal) => {
                let msg = literal.to_string();
                match msg.as_str() {
                    "\"before\"" => check_time = Some(CheckTime::Before),
                    "\"after\"" => check_time = Some(CheckTime::After),
                    "\"before_and_after\"" => check_time = Some(CheckTime::BeforeAndAfter),
                    _ => panic!("Invalid check time: {}, expected one of: \"before\", \"after\", \"before_and_after\"", msg)
                }
            }
            _ => {}
        }
    }

    let check_time = check_time.unwrap_or(CheckTime::BeforeAndAfter);

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

    let call_invariant_after = match check_time {
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
