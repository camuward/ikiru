use std::fmt::Display;
use std::ops::Range;

use proc_macro2::{Ident, Span, TokenStream};
use syn::spanned::Spanned;
use syn::{Error, MetaNameValue, Result};

pub fn mk_err<T>(span: &(impl Clone + quote::ToTokens), msg: impl Display) -> Result<T> {
    Err(Error::new_spanned(span.clone(), msg))
}

/// Whether the [`syn::Path`] is equal to the given identifier.
pub fn path_is(path: &syn::Path, ident: &str) -> bool {
    path.segments.len() == 1 && path.segments[0].ident == ident
}

/// Gets the types of all fields of a register struct.
pub fn field_types(fields: &syn::Fields) -> Result<Vec<Ident>> {
    fn field_ty(field: &syn::Field) -> Result<Ident> {
        match &field.ty {
            syn::Type::Path(p) if p.path.segments.len() == 1 => {
                Ok(p.path.segments[0].ident.clone())
            }
            _ => Err(Error::new_spanned(
                &field.ty,
                "expected bool/u8/u16/u32/i8/i16/i32",
            )),
        }
    }

    fields.iter().map(field_ty).collect()
}

// Parses the `key = "value"` pairs from `#[reg(...)]` attributes.
pub fn parse_index_and_width<'a>(
    index: &mut u32,
    ty: RegTy,
    attrs: impl IntoIterator<Item = &'a syn::Attribute>,
) -> Result<Range<u32>> {
    use syn::punctuated::Punctuated;
    use syn::{Attribute, Meta::List, MetaList, MetaNameValue, Path, Token};

    const DUPLICATE_WIDTH: &str = "duplicate `#[reg(width = ...)]` attribute";
    const INVALID_WIDTH: &str = r#"expected `#[reg(width = "n")]` where 1 <= n <= 32"#;
    const DUPLICATE_IDX: &str = "duplicate `#[reg(index = ...)]` attribute";
    const INVALID_RANGE: &str = r#"expected `#[reg(index = "i..j")]`"#;
    const END_BEFORE_START: &str = r#"expected `i < j` in `#[reg(index = "i..j")]`"#;
    const IDX_RANGE_WIDTH: &str =
        "cannot specify both `#[reg(index = ...)]` and `#[reg(width = ...)]`";
    const INVALID_IDX: &str = r#"expected `#[reg(index = "i")]` or `#[reg(index = "i..j")]`"#;
    const INVALID_ATTR: &str = "expected `#[reg(index = ...)]` or `#[reg(width = ...)]`";

    let mut attrs = attrs.into_iter();

    fn parse_meta(tokens: &TokenStream) -> Result<impl Iterator<Item = MetaNameValue>> {
        struct ListContents(Punctuated<MetaNameValue, Token![,]>);
        impl syn::parse::Parse for ListContents {
            fn parse(input: syn::parse::ParseStream) -> Result<Self> {
                Punctuated::parse_terminated(input).map(ListContents)
            }
        }

        Ok(syn::parse2::<ListContents>(tokens.clone())?.0.into_iter())
    }

    let kv_pairs = std::iter::from_fn(|| {
        // find next `#[reg(...)]` attribute
        attrs.find_map(|Attribute { meta, .. }| {
            // if attribute is `#[attr(...)]`
            if let List(MetaList { path, tokens, .. }) = meta {
                if path_is(path, "reg") {
                    return Some(parse_meta(tokens));
                }
            }

            None
        })
    })
    .collect::<Result<Vec<_>>>()?
    .into_iter()
    .flat_map(|kv_pairs| kv_pairs);

    let mut _overlap = 0u32; // TODO: check for overlap between fields
    let (mut idx, mut wid) = (None, None); // ensure no duplicate attributes
    for MetaNameValue { path, value, .. } in kv_pairs {
        use syn::{Expr, ExprLit, Lit::Int};

        // if the attribute is `#[reg(width = "n")]`
        if path_is(&path, "width") {
            match wid {
                Some(_) => mk_err(&path, DUPLICATE_WIDTH),
                None => match value {
                    Expr::Lit(ExprLit { lit: Int(int), .. }) => {
                        wid.replace(int.base10_parse()?);
                        Ok(())
                    }
                    _ => mk_err(&value, INVALID_WIDTH),
                },
            }
        // if the attribute is `#[reg(index = "i")]` or `#[reg(index = "i..j")]`
        } else if path_is(&path, "index") {
            match idx {
                Some(_) => mk_err(&path, DUPLICATE_IDX),
                None => match value {
                    // `#[reg(index = "i")]`
                    Expr::Lit(ExprLit {
                        lit: Int(ref int), ..
                    }) => {
                        idx.replace(int.base10_parse()?);
                        Ok(())
                    }
                    // `#[reg(index = "i..j")]`
                    Expr::Range(ref range) => {
                        // range must be half-open
                        let (start, end) = match range.limits {
                            syn::RangeLimits::HalfOpen(..) => {
                                let get_lit = |expr: &Option<Box<Expr>>| match expr.as_deref() {
                                    Some(Expr::Lit(ExprLit { lit: Int(int), .. })) => {
                                        Ok(int.base10_parse::<u32>()?)
                                    }
                                    _ => mk_err(&value, INVALID_RANGE),
                                };

                                Ok((get_lit(&range.start)?, get_lit(&range.end)?))
                            }
                            _ => mk_err(&value, INVALID_RANGE),
                        }?;

                        if start >= end {
                            return mk_err(&value, END_BEFORE_START);
                        }

                        match wid.replace(end - start) {
                            Some(_) => mk_err(&value, IDX_RANGE_WIDTH),
                            None => {
                                idx.replace(start);
                                Ok(())
                            }
                        }
                    }
                    _ => mk_err(&value, INVALID_IDX),
                },
            }?;

            if wid.is_some_and(|wid| wid > ty.width()) {
                return mk_err(
                    &value,
                    format!("width {} exceeds {}", wid.unwrap(), ty.width()),
                );
            }

            if let Some(end @ 33..) = idx.zip(wid).map(|(idx, wid)| idx + wid) {
                return mk_err(
                    &value,
                    format!("range {}..{end} exceeds 0..32", idx.unwrap()),
                );
            }

            Ok(())
        } else {
            mk_err(&path, INVALID_ATTR)
        }?
    }

    let wid = wid.unwrap_or(ty.width());
    let idx = idx.unwrap_or(*index);
    *index += wid;

    Ok(idx..idx + wid)
}

pub fn register_derive(ast: syn::DeriveInput) -> Result<TokenStream> {
    use syn::{Data::Struct, DataStruct};

    let name = &ast.ident;

    let fields = match &ast.data {
        Struct(DataStruct { fields, .. }) => fields,
        _ => return Err(Error::new_spanned(ast, "expected a struct")),
    };

    let f_ty = field_types(fields)?;

    // iterate over the fields, collecting the field names and their bitmasks
    let mut index: u32 = 0;
    let mut overlap: u32 = 0;
    let mut f_ident: Vec<&Ident> = vec![];
    let mut f_index: Vec<u32> = Vec::new();
    let mut f_width: Vec<u32> = Vec::new();

    for (syn::Field { attrs, ident, .. }, ty) in fields.iter().zip(&f_ty) {
        let range = parse_index_and_width(&mut index, RegTy::try_from(ty)?, attrs)?;
        let index = range.start;
        let width = range.end - range.start;

        f_index.push(index);
        f_width.push(width);

        let mask = ((1u32 << width) - 1) << index;
        if overlap & mask != 0 {
            return Err(Error::new_spanned(
                fields,
                "fields must not overlap in the register",
            ));
        } else {
            overlap |= mask;
        }

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
                #[doc = concat!("Set the value of `", stringify!(#f_ident), "`.")]
                pub fn #setter (&mut self, value: #f_ty) {
                    assert!(value < (1 << #f_width));  // Ensure value fits within width
                    self.#f_ident &= (1 << #f_width) - 1;  // Clear the bits
                    self.#f_ident |= value;
                }

                #[doc = concat!("Get the value of `", stringify!(#f_ident), "`.")]
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

#[derive(Debug)]
struct Reg {
    pub name: Ident,
    pub ty: RegTy,
    pub index: usize,
    pub width: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegTy {
    Bool,
    U8,
    U16,
    U32,
    I8,
    I16,
    I32,
}

impl RegTy {
    /// The default width of a register of this type
    pub const fn width(&self) -> u32 {
        match self {
            RegTy::Bool => 1,
            RegTy::U8 | RegTy::I8 => 8,
            RegTy::U16 | RegTy::I16 => 16,
            RegTy::U32 | RegTy::I32 => 32,
        }
    }
}

impl TryFrom<&Ident> for RegTy {
    type Error = Error;

    fn try_from(ident: &Ident) -> std::result::Result<Self, Self::Error> {
        match ident.to_string().as_str() {
            "bool" => Ok(RegTy::Bool),
            "u8" => Ok(RegTy::U8),
            "u16" => Ok(RegTy::U16),
            "u32" => Ok(RegTy::U32),
            "i8" => Ok(RegTy::I8),
            "i16" => Ok(RegTy::I16),
            "i32" => Ok(RegTy::I32),
            _ => Err(syn::Error::new_spanned(
                ident,
                "expected bool/u8/u16/u32/i8/i16/i32",
            )),
        }
    }
}
