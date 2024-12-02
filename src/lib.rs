// proc macro crate
use proc_macro::TokenStream;
use quote::quote;

// for enum, we'd like to generate From impls for each variant
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    print!("{:#?}", input);

    // get teh ident
    let ident = input.ident;
    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(e) => e.variants,
        _ => panic!("EnumFrom can only be derived for enums"),
    };
    // for each variant, get the ident and fileds
    let (idents, fields): (Vec<_>, Vec<_>) = variants
        .into_iter()
        .map(|v| (v.ident, v.fields))
        .unzip();

    // quote return proc-macro2 TokenStream so we need to convert it to TokenStream
    quote! {
        impl From<#ident> for String {
            fn from(s: #ident) -> String {
                match s {
                    #(#idents(f) => f.into(),)*
                }
            }
        }
    }.into()
}
