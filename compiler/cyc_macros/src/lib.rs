use proc_macro::TokenStream;

mod diagnostic;

#[proc_macro_derive(Diag, attributes(diag, primary_span, label, note))]
pub fn derive_diag(input: TokenStream) -> TokenStream {
    diagnostic::derive(input.into()).into()
}
