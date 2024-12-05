use auto::{process_auto_debug, process_auto_deref};
// proc macro crate
use quote::quote;

mod auto;

// for enum, we'd like to generate From impls for each variant
#[allow(unused)]
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    print!("{:#?}", input);

    // get the ident
    let ident = input.ident;
    // get generics
    let generics = input.generics;
    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works on enums"),
    };
    // for each variant, get the ident and fields
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        impl #generics  From<#ty> for #ident #generics {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }

            _ => quote! {},
        }
    });

    // quote return proce-macro2 TokenTtream so we need to convert it to TokenStream
    quote! {
        #(#from_impls)*
    }.into()
}


#[proc_macro_derive(AutoDeref, attributes(deref))]
pub fn derive_auto_deref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  
  process_auto_deref(input).into()
}

#[proc_macro_derive(AutoDebug, attributes(debug))]
pub fn derive_auto_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  
  process_auto_debug(input).into()
}