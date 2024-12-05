use std::process::id;

use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use syn::DeriveInput;
use quote::quote;

#[derive(Debug, FromDeriveInput)]
struct AutoDerefInfo {
  ident: syn::Ident,
  generics: syn::Generics,
  data: Data<(), FiledsInfo>,
  attrs: Vec<syn::Attribute>,
}

#[derive(Debug, FromField)]
struct FiledsInfo {
  ident: Option<syn::Ident>,
  ty: syn::Type,
  attrs: Vec<syn::Attribute>,
}

pub(crate) fn process_auto_deref(input: DeriveInput) -> TokenStream {
  let AutoDerefInfo { 
    ident, 
    generics, 
    data: Data::Struct(fields) ,
    attrs,
  } = AutoDerefInfo::from_derive_input(&input).unwrap() 
  else {
    panic!("AutoDeref can only be derived for structs")
  };

  println!("{:?} {:?}", ident, generics);
  println!("{:#?}", fields);
  println!("{:#?}", attrs);
  
  quote!{

  }
}

pub(crate) fn process_auto_debug(input: DeriveInput) -> TokenStream {
  
  
  quote!{

  }
}