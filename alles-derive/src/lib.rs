extern crate proc_macro;
use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse_macro_input, spanned::Spanned, DataEnum, DataStruct, DeriveInput, Fields, Generics, Ident,
};

#[proc_macro_error]
#[proc_macro_derive(Alles)]
pub fn alles_derive(input: TS) -> TS {
    let derive_input: DeriveInput = parse_macro_input!(input);

    let imp = match derive_input.data {
        syn::Data::Struct(st) => derive_struct(derive_input.ident, st, derive_input.generics),
        syn::Data::Enum(en) => derive_enum(derive_input.ident, en, derive_input.generics),
        syn::Data::Union(c) => abort!(c.union_token, "Unions are not supported"),
    };

    imp.into()
}

fn derive_enum(ident: Ident, en: DataEnum, generics: Generics) -> TokenStream {
    let (impl_gen, ty_gen, where_gen) = generics.split_for_impl();
    if en.variants.is_empty() {
        abort!(
            ident,
            "Empty enums cannot be instantiated, so Alles cannot be derived for it."
        );
    }
    let all_variants = en.variants.iter().map(generate_build_for_variant);

    quote! {
        impl #impl_gen alles::Alles for #ident #ty_gen #where_gen {
            fn generate() -> impl core::iter::Iterator<Item = Self> + Clone {
                let fields = std::iter::empty::<Self>();

                #(
                    let fields = fields.chain(#all_variants);
                )*

                fields
            }
        }
    }
}

fn derive_struct(ident: Ident, st: DataStruct, generics: Generics) -> TokenStream {
    let (impl_gen, ty_gen, where_gen) = generics.split_for_impl();

    let fields_init = generate_init_for_fields(&st.fields);
    let fields_build = match &st.fields {
        syn::Fields::Named(fields) => {
            if fields.named.is_empty() {
                quote! {
                    core::iter::once(Self { })
                }
            } else {
                let fields_idents = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect::<Vec<_>>();

                quote! {
                    alles::private::iproduct!( #( #fields_idents ),* ).map(|( #( #fields_idents ),* )| {
                        Self { #( #fields_idents ),* }
                    })
                }
            }
        }
        syn::Fields::Unnamed(fields) => {
            if fields.unnamed.is_empty() {
                quote! {
                    core::iter::once(Self { })
                }
            } else {
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

                quote! {
                    alles::private::iproduct!( #( #fields_idents ),* ).map(|( #( #fields_idents ),* )| {
                        Self { #( #fields_idx: #fields_idents ),* }
                    })
                }
            }
        }
        syn::Fields::Unit => quote! {
            core::iter::once(Self)
        },
    };

    quote! {
        impl #impl_gen alles::Alles for #ident #ty_gen #where_gen {
            fn generate() -> impl core::iter::Iterator<Item = Self> + Clone {
                #fields_init

                #fields_build
            }
        }
    }
}

fn generate_build_for_variant(variant: &syn::Variant) -> TokenStream {
    let variant_ident = &variant.ident;
    let fields_init = generate_init_for_fields(&variant.fields);
    let fields_build = match &variant.fields {
        syn::Fields::Named(fields) => {
            if fields.named.is_empty() {
                quote! {
                    core::iter::once(Self:: #variant_ident { })
                }
            } else {
                let fields_idents = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect::<Vec<_>>();

                quote! {
                    alles::private::iproduct!( #( #fields_idents ),* ).map(|( #( #fields_idents ),* )| {
                        Self:: #variant_ident { #( #fields_idents ),* }
                    })
                }
            }
        }
        syn::Fields::Unnamed(fields) => {
            if fields.unnamed.is_empty() {
                quote! {
                    core::iter::once(Self:: #variant_ident { })
                }
            } else {
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

                quote! {
                    alles::private::iproduct!( #( #fields_idents ),* ).map(|( #( #fields_idents ),* )| {
                        Self:: #variant_ident { #( #fields_idx: #fields_idents ),* }
                    })
                }
            }
        }
        syn::Fields::Unit => quote! {
            core::iter::once(Self:: #variant_ident)
        },
    };
    quote! {
        {
            #fields_init
            #fields_build
        }
    }
}

fn generate_init_for_fields(fields: &Fields) -> TokenStream {
    match fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| {
                let fident = f.ident.as_ref().unwrap();
                let fty = &f.ty;

                quote_spanned! {f.ty.span()=>
                    let #fident = <#fty as alles::Alles>::generate();
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(idx, f)| {
                let fident = format_ident!("_{idx}");
                let fty = &f.ty;

                quote_spanned! {f.ty.span()=>
                    let #fident = <#fty as alles::Alles>::generate();
                }
            })
            .collect(),
        syn::Fields::Unit => quote! {},
    }
}
