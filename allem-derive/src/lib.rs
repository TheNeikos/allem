extern crate proc_macro;
use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, DataEnum, DataStruct, DeriveInput, Error, Expr,
    Field, Fields, Generics, Ident, Token,
};

#[proc_macro_derive(Alles, attributes(alles))]
pub fn alles_derive(input: TS) -> TS {
    let derive_input: DeriveInput = parse_macro_input!(input);

    let imp = match derive_input.data {
        syn::Data::Struct(st) => derive_struct(derive_input.ident, st, derive_input.generics),
        syn::Data::Enum(en) => derive_enum(derive_input.ident, en, derive_input.generics),
        syn::Data::Union(c) => Err(Error::new(c.union_token.span(), "Unions are not supported")),
    }
    .unwrap_or_else(Error::into_compile_error);

    imp.into()
}

fn derive_enum(ident: Ident, en: DataEnum, generics: Generics) -> Result<TokenStream, Error> {
    let (impl_gen, ty_gen, where_gen) = generics.split_for_impl();
    if en.variants.is_empty() {
        return Err(Error::new(
            ident.span(),
            "Empty enums cannot be instantiated, so Alles cannot be derived for it.",
        ));
    }
    let all_variants = en
        .variants
        .iter()
        .map(generate_build_for_variant)
        .collect::<Result<Vec<TokenStream>, Error>>()?;

    Ok(quote! {
        impl #impl_gen allem::Alles for #ident #ty_gen #where_gen {
            fn generate() -> impl core::iter::Iterator<Item = Self> + Clone {
                let fields = std::iter::empty::<Self>();

                #(
                    let fields = fields.chain(#all_variants);
                )*

                fields
            }
        }
    })
}

fn derive_struct(ident: Ident, st: DataStruct, generics: Generics) -> Result<TokenStream, Error> {
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
                    allem::private::iproduct!( #( #fields_idents ),* ).map(|( #( #fields_idents ),* )| {
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
                    allem::private::iproduct!( #( #fields_idents ),* ).map(|( #( #fields_idents ),* )| {
                        Self { #( #fields_idx: #fields_idents ),* }
                    })
                }
            }
        }
        syn::Fields::Unit => quote! {
            core::iter::once(Self)
        },
    };

    Ok(quote! {
        impl #impl_gen allem::Alles for #ident #ty_gen #where_gen {
            fn generate() -> impl core::iter::Iterator<Item = Self> + Clone {
                #fields_init

                #fields_build
            }
        }
    })
}

fn generate_build_for_variant(variant: &syn::Variant) -> Result<TokenStream, Error> {
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
                    allem::private::iproduct!( #( #fields_idents ),* ).map(|( #( #fields_idents ),* )| {
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
                    allem::private::iproduct!( #( #fields_idents ),* ).map(|( #( #fields_idents ),* )| {
                        Self:: #variant_ident { #( #fields_idx: #fields_idents ),* }
                    })
                }
            }
        }
        syn::Fields::Unit => quote! {
            core::iter::once(Self:: #variant_ident)
        },
    };
    Ok(quote! {
        {
            #fields_init
            #fields_build
        }
    })
}

fn generate_init_for_fields(fields: &Fields) -> TokenStream {
    let combine = |fident, f: &Field| -> TokenStream {
        let fty = &f.ty;
        let fattrs = match parse_field_attributes(&f.attrs) {
            Ok(fattrs) => fattrs,
            Err(err) => {
                let err_stream = err.into_compile_error();
                return quote_spanned! {f.ty.span()=>
                    let #fident = #err_stream;
                };
            }
        };

        let gen = fattrs
            .with_values
            .map(|with_values| {
                quote! { (#with_values).into_iter().map(|i| core::convert::Into::into(i)) }
            })
            .unwrap_or_else(|| quote!(<#fty as allem::Alles>::generate()));

        let and_values = fattrs
            .and_values
            .map(|and_values| {
                quote! { (#and_values).into_iter().map(|i| core::convert::Into::into(i)) }
            })
            .unwrap_or_else(|| quote! { core::iter::empty() });

        quote_spanned! {f.ty.span()=>
            let #fident = (#gen).chain(#and_values);
        }
    };

    match fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| {
                let fident = f.ident.clone().unwrap();
                combine(fident, f)
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(idx, f)| {
                let fident = format_ident!("_{idx}");
                combine(fident, f)
            })
            .collect(),
        syn::Fields::Unit => quote! {},
    }
}

struct FieldAttributes {
    with_values: Option<Expr>,
    and_values: Option<Expr>,
}

fn parse_field_attributes(attrs: &[Attribute]) -> Result<FieldAttributes, syn::Error> {
    let mut field_attrs = FieldAttributes {
        with_values: None,
        and_values: None,
    };

    for attr in attrs {
        if attr.path().is_ident("alles") {
            attr.parse_nested_meta(|meta| {
                // used like `with_values = <expr>`
                if meta.path.is_ident("with_values") {
                    meta.input.parse::<Token![=]>()?;
                    let values = meta.input.parse()?;
                    field_attrs.with_values = Some(values);
                    return Ok(());
                }

                // used like `and_values = <expr>`
                if meta.path.is_ident("and_values") {
                    meta.input.parse::<Token![=]>()?;
                    let values = meta.input.parse()?;
                    field_attrs.and_values = Some(values);
                    return Ok(());
                }

                Err(meta.error("Unknown attribute kind for the Alles derive"))
            })?;
        }
    }

    Ok(field_attrs)
}
