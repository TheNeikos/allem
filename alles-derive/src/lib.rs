extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Alles)]
pub fn alles_derive(input: TokenStream) -> TokenStream {
    input
}
