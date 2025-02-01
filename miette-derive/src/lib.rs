use quote::ToTokens;
use syn::parse_macro_input;

use diagnostic::Diagnostic;

mod code;
mod diagnostic;
mod diagnostic_arg;
mod diagnostic_source;
mod fmt;
mod forward;
mod help;
mod label;
mod related;
mod severity;
mod source_code;
mod url;
mod utils;

#[proc_macro_derive(
    Diagnostic,
    attributes(diagnostic, source_code, label, related, help, diagnostic_source)
)]
pub fn derive_diagnostic(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Diagnostic);
    input.into_token_stream().into()
}
