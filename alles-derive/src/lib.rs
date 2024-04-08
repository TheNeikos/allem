extern crate proc_macro;
use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DataStruct, DeriveInput, Generics, Ident};

#[proc_macro_error]
#[proc_macro_derive(Alles)]
pub fn alles_derive(input: TS) -> TS {
    let derive_input: DeriveInput = parse_macro_input!(input);

    let imp = match derive_input.data {
        syn::Data::Struct(st) => derive_struct(derive_input.ident, st, derive_input.generics),
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(c) => abort!(c.union_token, "Unions are not supported"),
    };

    imp.into()
}

fn derive_struct(ident: Ident, st: DataStruct, generics: Generics) -> TokenStream {
    let (impl_gen, ty_gen, where_gen) = generics.split_for_impl();
    match st.fields {
        syn::Fields::Named(fields) => {
            let fields_init = fields.named.iter().map(|f| {
                let fident = f.ident.as_ref().unwrap();
                let fty = &f.ty;

                quote_spanned! {f.span()=>
                    let #fident = <#fty as alles::Alles>::generate();
                }
            });

            let fields_idents = fields
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap())
                .collect::<Vec<_>>();

            let fields_build = quote! {
                alles::private::iproduct!( #( #fields_idents ),* ).map(|( #( #fields_idents ),* )| {
                    Self { #( #fields_idents ),* }
                })
            };

            if fields.named.is_empty() {
                quote! {
                    impl #impl_gen alles::Alles for #ident #ty_gen #where_gen {
                        fn generate() -> impl core::iter::Iterator<Item = Self> + Clone {
                            core::iter::once(Self { })
                        }
                    }
                }
            } else {
                quote! {
                    impl #impl_gen alles::Alles for #ident #ty_gen #where_gen {
                        fn generate() -> impl core::iter::Iterator<Item = Self> + Clone {
                            #( #fields_init )*

                            #fields_build
                        }
                    }
                }
            }
        }
        syn::Fields::Unnamed(fields) => {
            let fields_init = fields.unnamed.iter().enumerate().map(|(idx, f)| {
                let fident = format_ident!("_{idx}");
                let fty = &f.ty;

                quote_spanned! {f.span()=>
                    let #fident = <#fty as alles::Alles>::generate();
                }
            });

            let fields_idents = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(idx, _f)| format_ident!("_{idx}"))
                .collect::<Vec<_>>();

            let fields_idx = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(idx, _)| syn::Index::from(idx));

            let fields_build = quote! {
                alles::private::iproduct!( #( #fields_idents ),* ).map(|( #( #fields_idents ),* )| {
                    Self { #( #fields_idx: #fields_idents ),* }
                })
            };

            if fields.unnamed.is_empty() {
                quote! {
                    impl #impl_gen alles::Alles for #ident #ty_gen #where_gen {
                        fn generate() -> impl core::iter::Iterator<Item = Self> + Clone {
                            core::iter::once(Self { })
                        }
                    }
                }
            } else {
                quote! {
                    impl #impl_gen alles::Alles for #ident #ty_gen #where_gen {
                        fn generate() -> impl core::iter::Iterator<Item = Self> + Clone {
                            #( #fields_init )*

                            #fields_build
                        }
                    }
                }
            }
        }
        syn::Fields::Unit => quote! {

            impl #impl_gen alles::Alles for #ident #ty_gen #where_gen {
                fn generate() -> impl core::iter::Iterator<Item = Self> + Clone {
                    core::iter::once(Self)
                }
            }
        },
    }
}
