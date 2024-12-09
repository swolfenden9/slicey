use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

pub fn impl_sliced(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let sliced_name = format_ident!("Sliced{}", name);

    // Generate the new Kind type and the alias type
    let expanded = quote! {
        // Generate the alias
        type #sliced_name<'src> = slicey::Sliced<'src, #name>;
    };

    TokenStream::from(expanded)
}
