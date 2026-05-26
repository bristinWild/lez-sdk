use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod, Visibility};

/// Marks a module as a LEZ program.
/// Generates the program entrypoint and instruction router.
#[proc_macro_attribute]
pub fn program(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemMod);
    let mod_name = &input.ident;
    let vis = &input.vis;
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
        #vis mod #mod_name {
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

/// Mark a struct as an event payload for IDL generation.
///
/// The `discriminant` argument must match the value passed to `emit_event()`.
///
/// # Example
/// ```rust,ignore
/// #[event(discriminant = 1)]
/// pub struct InsufficientFunds {
///     pub requested: u128,
///     pub available: u128,
/// }
/// ```
///
/// This attribute is a no-op at compile time; it is consumed solely by the
/// IDL generator alongside `#[account_type]`.
#[proc_macro_attribute]
pub fn event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
