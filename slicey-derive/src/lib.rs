use proc_macro::TokenStream;
use sliced::impl_sliced;
use spanned::impl_spanned;

mod sliced;
mod spanned;

#[proc_macro_derive(Sliced)]
pub fn sliced_derive(input: TokenStream) -> TokenStream {
    impl_sliced(input)
}

#[proc_macro_derive(Spanned)]
pub fn spanned_derive(input: TokenStream) -> TokenStream {
    impl_spanned(input)
}
