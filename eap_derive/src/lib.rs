extern crate proc_macro;
use attributes::create_impl;
use proc_macro::TokenStream;
use syn::DeriveInput;

mod attributes;

#[proc_macro_derive(Config, attributes(var))]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    let fields = match ast.data {
        syn::Data::Struct(ref data_struct) => {
            match data_struct.fields {
                syn::Fields::Named(ref fields) => &fields.named,
                _ => panic!("Only named fields are supported")
            }
        },
        _ => panic!("Only structs are supported")
    };

    let ts = create_impl(name, fields);

    //panic!("{:?}", ts.to_string());

    ts.into()
}
