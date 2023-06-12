use proc_macro::TokenStream;

mod derive_reg;
mod tests;

#[proc_macro_derive(Reg, attributes(reg))]
pub fn register_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse_macro_input!(input);
    
    match derive_reg::register_derive(ast) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
