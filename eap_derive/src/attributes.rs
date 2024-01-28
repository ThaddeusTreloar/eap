use core::panic;
use std::any::{Any, TypeId};

use proc_macro::TokenStream;
use syn::{meta::ParseNestedMeta, parse_quote, punctuated::Punctuated, token::Comma, DeriveInput, Expr, Field, GenericArgument, Ident, LitStr, PathArguments, Token};

fn extract_ident(field: &Field) -> &Ident {
    match &field.ident {
        Some(ident) => ident,
        None => panic!("Invalid attribute")
    }
}

fn extract_name_literal(field: &Field) -> String {
    match &field.ident {
        Some(ident) => ident.to_string(),
        None => panic!("Invalid attribute")
    }
}

pub fn create_impl(
    name: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {
    let field_results: Vec<_> = fields.iter().map(ConfigField::try_from)
        .collect();

    if let Some(Err(e)) = field_results.iter().find(|res| res.is_err()) {
        panic!("{e}");
    }

    let fields: Vec<_> = field_results.into_iter()
        .map(Result::unwrap)
        .map(ConfigField::build_backend_retrieval_syntax)
        .collect();

    parse_quote!(
        impl Config for #name {
            fn parse<T: Environment>(backend: T) -> Self {
                Self {
                    #(#fields),*
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

struct ConfigField {
    pub name: Ident,
    pub ty: syn::Type,
    pub attributes: Vec<EapAttribute>
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum ParseFieldError {
    #[error("Invalid attribute: {0}")]
    InvalidAttribute(#[from] ParseAttributeError), // Todo: better feedback
    #[error("Invalid value")]
    InvalidValue
}

impl ConfigField {
    fn build_backend_retrieval_syntax(self) -> proc_macro2::TokenStream {
        let name = self.name.clone();
        let name_literal = self.name.to_string();
        let ty = self.ty.clone();
        let value = self.attributes
            .iter()
            .find(|a| matches!(a.name, AttributeKind::Var(VarAttribute::Default)))
            .map(|a| &a.value);

        match value {
            Some(AttributeValue::Literal(_)) => unimplemented!(),
            Some(AttributeValue::Expr(expr)) => parse_quote!(#name: backend.try_get_or::<#ty>(#name_literal, #expr)),
            None => {
                match ty.clone() {
                    syn::Type::Path(path) => {
                        let path = path.path;
                        let mut smegments: Vec<_> = path.segments.iter().collect();

                        //panic!("{:?}", smegments);

                        let mut segs = path.segments.iter();

                        let first_type = segs.next()
                        .expect("Attempted to take first type segment, found none");
                    
                    
                        if first_type.ident.clone() == "Option" {
                            let PathArguments::AngleBracketed(args) = first_type.arguments.clone() else {
                                panic!("Expected angle bracketed type arguments")
                            };

                            let GenericArgument::Type(p) = args.args.iter().next().expect("Expected one type argument") else {
                                panic!("Expected one type argument")
                            };

                            parse_quote!(#name: backend.try_get::<#p>(#name_literal).expect(std::stringify!(Failed to parse <#p> from variable #name_literal)))
                        } else {
                            unimplemented!("Required fields are not yet supported")
                            //parse_quote!(#name: backend.try_get::<#ty>(#name_literal).unwrap()),
                        }
                    },
                    _ => unimplemented!("Required fields are not yet supported")
                }
                /*if ty.type_id() == TypeId::of::<Option<...>>() {
                    parse_quote!(#name: backend.try_get::<#ty>(#name_literal))
                } else {
                    unimplemented!("Required fields are not yet supported")
                    //parse_quote!(#name: backend.try_get::<#ty>(#name_literal).unwrap()),
                }*/
            }
        }
    }
}

impl TryFrom<&Field> for ConfigField {
    type Error = ParseFieldError;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        let name = extract_ident(field);
        let ty = field.ty.clone();
        let attr_results: Vec<Result<EapAttribute, ParseAttributeError>> = field.attrs.iter()
            .map(
                EapAttribute::try_from
            ).collect();

        if let Some(Err(e)) = attr_results.iter().find(|res| res.is_err()) {
            return Err(e.clone().into())
        }

        let attrs: Vec<_> = attr_results.into_iter()
            .map(Result::unwrap)
            .collect();
        
        Ok(
            Self {
                name: name.clone(),
                ty,
                attributes: attrs,
            }
        )
    }
}

enum AttributeKind {
    Var(VarAttribute)
}

enum ValueKind {
    Optional,
    Required
}

enum VarAttribute {
    Default
}

#[derive(Clone)]
pub enum AttributeValue {
    Literal(LitStr),
    Expr(Expr)
}


pub struct EapAttribute {
    name: AttributeKind,
    value: AttributeValue
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum ParseAttributeError {
    #[error("Invalid attribute")]
    InvalidAttribute,
    #[error("Invalid value")]
    InvalidValue
}

impl TryFrom<ParseNestedMeta<'_>> for EapAttribute {
    type Error = ParseAttributeError;

    fn try_from(meta: ParseNestedMeta) -> Result<Self, Self::Error> {
        let mut name = None;
        let mut value = None;

        if meta.path.is_ident("default") {
            let content = meta.input;

            let assign = content.parse::<Token![=]>()
                .map_err(|_|ParseAttributeError::InvalidAttribute)?;

            let expr = content.parse::<Expr>().unwrap();
            name = Some(AttributeKind::Var(VarAttribute::Default));
            value = Some(AttributeValue::Expr(expr));
        } else {
            panic!("Invalid attribute")
        }
        
        Ok(Self {
            name: name.map_or(Err(ParseAttributeError::InvalidAttribute), Ok)?,
            value: value.map_or(Err(ParseAttributeError::InvalidValue), Ok)?
        })
    }
}

impl TryFrom<&syn::Attribute> for EapAttribute {
    type Error = ParseAttributeError;

    fn try_from(attr: &syn::Attribute) -> Result<Self, Self::Error> {
        let mut eap_attr: Result<EapAttribute, ParseAttributeError> = Err(ParseAttributeError::InvalidAttribute);

        let _ = attr.parse_nested_meta(
            |m| {
                eap_attr = EapAttribute::try_from(m);

                Ok(())
            }
        );

        eap_attr
    }
}
