use prettyplease;
use proc_macro2::TokenStream;

pub const DIM: &str = "D";
pub const RANK: &str = "Rank";

pub fn format_token_stream(tokens: TokenStream) -> syn::Result<String> {
    Ok(prettyplease::unparse(&syn::parse_file(
        &tokens.to_string(),
    )?))
}
