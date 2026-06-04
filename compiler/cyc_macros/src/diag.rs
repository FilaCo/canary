use proc_macro2::TokenStream;

pub fn derive(input: TokenStream) -> TokenStream {
    expand(input).unwrap_or_else(syn::Error::into_compile_error)
}

fn expand(input: TokenStream) -> syn::Result<TokenStream> {
    let syn::DeriveInput { ident, .. } = syn::parse2(input)?;
    todo!()
}
