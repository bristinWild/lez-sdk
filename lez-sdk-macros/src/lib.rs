use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod};

/// Marks a module as a LEZ program.
/// Generates the program entrypoint and instruction router.
#[proc_macro_attribute]
pub fn program(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemMod);
    let mod_name = &input.ident;
    let mod_content = &input.content;

    let items = match mod_content {
        Some((_, items)) => items,
        None => {
            return syn::Error::new_spanned(&input, "program module must have a body")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        pub mod #mod_name {
            #(#items)*
        }
    };

    TokenStream::from(expanded)
}

/// Marks a function as a LEZ program instruction handler.
/// Generates argument decoding from raw bytes.
#[proc_macro_attribute]
pub fn function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Marker only at this stage — routing is explicit via match in entrypoint
    item
}
