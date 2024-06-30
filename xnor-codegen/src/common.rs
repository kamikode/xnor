use prettyplease;
use proc_macro2;

pub const DIM: &str = "D";

pub fn format_token_stream(tokens: proc_macro2::TokenStream) -> String {
    prettyplease::unparse(&syn::parse_file(&tokens.to_string()).unwrap())
}
