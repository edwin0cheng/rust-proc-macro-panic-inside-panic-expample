extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer_macro(input: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
