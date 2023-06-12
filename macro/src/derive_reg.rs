use proc_macro2::{Ident, Span, TokenStream};
use syn::{Data::Struct, DataStruct, DeriveInput, Error};

fn field_types(fields: &syn::Fields) -> Result<Vec<Ident>, Error> {
    let field_type = |f: &syn::Field| match &f.ty {
        syn::Type::Path(p) if p.path.segments.len() == 1 => Ok(p.path.segments[0].ident.clone()),
        _ => Err(Error::new_spanned(
            &f.ty,
            "expected bool/u8/u16/u32/i8/i16/i32",
        )),
    };

    fields.iter().map(field_type).collect()
}

fn type_width(ty: Ident) -> Result<usize, Error> {
    todo!()
}

pub fn register_derive(ast: DeriveInput) -> Result<TokenStream, Error> {
    let name = &ast.ident;

    let fields = match &ast.data {
        Struct(DataStruct { fields, .. }) => fields,
        _ => return Err(Error::new_spanned(ast, "expected a struct")),
    };

    let f_ty = field_types(fields)?;
    let f_width = f_ty.iter().map(|_| todo!());

    // iterate over the fields, collecting the field names and their bitmasks
    let mut index: usize = 0;
    let mut overlap: u32 = 0;
    let mut f_ident: Vec<&Ident> = vec![];
    let mut f_index: Vec<usize> = Vec::new();
    let mut f_width: Vec<usize> = Vec::new();

    for syn::Field {
        attrs,
        vis,
        mutability,
        ident,
        colon_token,
        ty,
    } in fields
    {
        let mut width: Option<usize> = None;

        let attr_contents = {
            use syn::punctuated::Punctuated;
            use syn::{Attribute, Meta::List, MetaList, MetaNameValue, Path, Token};

            let mut attrs = attrs.iter();

            // find next `#[reg(...)]` attribute
            let mut find_reg = || {
                for Attribute { meta, .. } in attrs.by_ref() {
                    // if attribute is `#[attr(...)]`
                    if let List(MetaList { path, tokens, .. }) = meta {
                        // check if name matches `reg`
                        let path = path.segments.iter().map(|seg| &seg.ident);
                        if path.eq([&Ident::new("reg", Span::call_site())]) {
                            return Some(tokens);
                        }
                    }
                }

                None
            };

            struct ListContents(Punctuated<MetaNameValue, Token![,]>);
            impl syn::parse::Parse for ListContents {
                fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                    Punctuated::parse_terminated(input).map(ListContents)
                }
            }

            if let Some(reg) = find_reg() {
                todo!();

                if let Some(reg) = find_reg() {
                    return Err(Error::new_spanned(
                        reg,
                        "too many `#[reg(...)]` attributes (max 1)",
                    ));
                }
            }
        };

        // let type_width = || match ty {
        //     _ => todo!(),
        // };

        // enum Attrs {
        //     /// `#[reg(index = 0)]`
        //     Index(u32),
        //     /// `#[reg(index = 0..6)]`
        //     /// `#[reg(index = 0, width = 6)]`
        //     IndexWidth(u32, u32),
        //     /// `#[reg(width = 6)]`
        //     Width(u32),
        // }

        // let attrs = todo!();
        // let (index, width) = match attrs {
        //     Attrs::Index(idx) => (idx, type_width(ty)),
        // }

        match ident {
            Some(ident) => f_ident.push(ident),
            None => {
                return Err(Error::new_spanned(
                    fields,
                    "expected a struct with named fields",
                ));
            }
        }
    }

    let getter: Vec<_> = f_ident.iter().map(|field| format!("get_{field}")).collect();
    let setter: Vec<_> = f_ident.iter().map(|field| format!("set_{field}")).collect();

    Ok(quote::quote! {
        impl #name {
            #(
                #[doc = concat!("Set the value of `", stringify!(#f_ident), "`")]
                pub fn #setter (&mut self, value: #f_ty) {
                    assert!(value < (1 << #f_width));  // Ensure value fits within width
                    self.#f_ident &= (1 << #f_width) - 1;  // Clear the bits
                    self.#f_ident |= value;
                }

                pub fn #getter (&self) -> #f_ty {
                    let mask = (1 << #f_width) - 1;
                    self.#f_ident & mask
                }
            )*
        }

        impl Reg for #name {
            fn set(&mut self, value: u32) {
                #(
                    self. #setter ((value >> #f_index) & ((1 << #f_width) - 1));
                )*
            }

            fn get(&self) -> u32 {
                let mut val = 0;
                #(
                    val |= (self. #getter () as u32 << #f_index);
                )*
                val
            }
        }
    })
}
