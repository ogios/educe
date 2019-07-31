use super::super::TraitHandler;
use super::models::{TypeAttributeBuilder, FieldAttributeBuilder};

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data};

pub struct CloneUnionHandler;

impl TraitHandler for CloneUnionHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        let _ = TypeAttributeBuilder {
            enable_bound: false,
        }.from_clone_meta(meta);

        if let Data::Union(data) = &ast.data {
            for field in data.fields.named.iter() {
                let _ = FieldAttributeBuilder {
                    enable_clone: false
                }.from_attributes(&field.attrs, traits);
            }
        }

        let ident = &ast.ident;

        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        let compare_impl = quote! {
            impl #impl_generics core::clone::Clone for #ident #ty_generics #where_clause {
                fn clone(&self) -> Self {
                    *self
                }
            }
        };

        tokens.extend(compare_impl);
    }
}