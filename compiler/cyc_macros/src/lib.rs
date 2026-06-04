use proc_macro::TokenStream;

mod diag;

#[proc_macro_derive(Diag, attributes(diag, primary_span, label, note))]
pub fn derive_diag(input: TokenStream) -> TokenStream {
    diag::derive(input.into()).into()
}
