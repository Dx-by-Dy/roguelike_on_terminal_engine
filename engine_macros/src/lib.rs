use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields, Path, Token, parse_macro_input, punctuated::Punctuated};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    let expanded = quote! {
        impl Component for #ident {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Delegate, attributes(delegate))]
pub fn derive_delegate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let data = match input.data {
        syn::Data::Struct(s) => s,
        _ => {
            return syn::Error::new_spanned(&struct_name, "Delegate only works on structs")
                .to_compile_error()
                .into();
        }
    };

    let fields = match data.fields {
        Fields::Named(f) => f.named,
        _ => {
            return syn::Error::new_spanned(&struct_name, "Delegate requires named fields")
                .to_compile_error()
                .into();
        }
    };

    let mut impls = Vec::new();

    for field in fields {
        let field_name = field.ident.unwrap();

        for attr in field.attrs {
            if attr.path().is_ident("delegate") {
                let traits: Punctuated<Path, Token![,]> = attr
                    .parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
                    .expect("Failed to parse trait list in #[delegate(..)]");

                for trait_path in traits {
                    let macro_name = syn::Ident::new(
                        &format!(
                            "__delegate_{}",
                            trait_path.segments.last().unwrap().ident.to_string()
                        ),
                        trait_path.segments.last().unwrap().ident.span(),
                    );

                    impls.push(quote! {
                        impl #trait_path for #struct_name {
                            #macro_name!(#field_name);
                        }
                    });
                }
            }
        }
    }

    TokenStream::from(quote! { #(#impls)* })
}
