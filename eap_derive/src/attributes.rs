use proc_macro::TokenStream;
use syn::{parse_quote, punctuated::Punctuated, token::Comma, DeriveInput, Expr, Field, Ident, LitStr, Token};

pub fn create_impl(
    name: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {

    let field_names = fields.iter().map(|field| &field.ident);
    let field_literals = fields.iter().map(|field| match &field.ident {
        Some(field) => field.to_string(),
        None => panic!("Invalid attribute")
    });
    let field_defaults = fields.iter()
        .map(|field| (field, &field.ty))
        .map(
            |(field, ty)| (field.attrs.iter().find(
                |attr| attr.path().is_ident("var")
            ), ty)
        ).map(
            |(attr, _ty)| match attr {
                Some(attr) => {
                    let mut def = None;
                    attr.parse_nested_meta(
                        |meta| {
                            if meta.path.is_ident("default") {
                                let content = meta.input;
                                let assign = content.parse::<Token![=]>()?;
                                def = Some(content.parse::<Expr>().unwrap());
                                Ok(())
                            } else {
                                panic!("Invalid attribute")
                            }
                        }
                    ).unwrap();
                    
                    def.unwrap()
                },
                None => panic!("Invalid attribute")
            }
        );
    let field_types = fields.iter().map(|field| &field.ty);

    parse_quote!(
        impl Config for #name {
            fn parse<T: Environment>(backend: T) -> Self {
                Self {
                    #(#field_names: backend.try_get_or::<#field_types>(#field_literals, #field_defaults)),*
                }
            }
        }

        impl <E: Environment> From<E> for #name {
            fn from(env: E) -> Self {
                Self::parse(env)
            }
        }
    )
}

/*
pub struct EapAttribute {
    pub name: AttributeKind,
    pub value: AttributeValue
}

impl EapAttribute {
    pub fn parse_attributes(attrs: Vec<syn::Attribute>) -> Result<Vec<Self>, syn::Error> {
        attrs.into_iter()
            .filter(|attr| attr.path.is_ident("default"))
            .map(|attr| {
                let meta = attr.parse_meta().unwrap();
                let name = AttributeKind::Default;
                let value = match meta {
                    syn::Meta::NameValue(nv) => {
                        let lit = nv.lit;
                        match lit {
                            syn::Lit::Str(lit_str) => AttributeValue::Literal(lit_str),
                            syn::Lit::Int(lit_int) => AttributeValue::Expr(syn::parse2(lit_int.into_token_stream()).unwrap()),
                            _ => panic!("Invalid value")
                        }
                    },
                    _ => panic!("Invalid attribute")
                };
                Self {
                    name,
                    value
                }
            })
            .collect()
    }
}

pub enum AttributeValue {
    Literal(LitStr),
    Expr(Expr)
}

pub enum AttributeKind {
    Default
}

impl AttributeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default"
        }
    }
}
 */