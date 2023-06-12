#![cfg(test)]
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::Meta::List;
use syn::{AttrStyle, Attribute, Ident, MetaList, Path, PathArguments, PathSegment};

#[test]
fn overlap() {
    // fn create_attr(params: TokenStream) -> Attribute {
    //     Attribute {
    //         pound_token: Default::default(),
    //         style: AttrStyle::Outer,
    //         bracket_token: Default::default(),
    //         meta: List(MetaList {
    //             path: Path {
    //                 leading_colon: None,
    //                 segments: Punctuated::from_iter([PathSegment {
    //                     ident: Ident::new("reg", Span::call_site()),
    //                     arguments: PathArguments::None,
    //                 }]),
    //             },
    //             delimiter: syn::MacroDelimiter::Paren(Default::default()),
    //             tokens: quote::quote!(#params),
    //         }),
    //     }
    // }

    // let attrs = [
    //     create_attr(r#"index = "2..6""#),
    //     create_attr(r#"index = "4..8""#),
    // ];

    // let (idx, wid) = crate::derive_reg::field_attr_list(attrs).unwrap_err();
}
