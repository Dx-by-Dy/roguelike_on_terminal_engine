use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, Fields, ItemFn, LitInt, Path, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

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

struct Args {
    pub init_gtr: bool,
    pub set_in_datamaster: bool,
    pub timestamp: Option<usize>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Args {
                init_gtr: false,
                set_in_datamaster: false,
                timestamp: None,
            });
        }

        let mut args = Args {
            init_gtr: false,
            set_in_datamaster: false,
            timestamp: None,
        };

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;

            match &*ident.to_string() {
                "init_gtr" => args.init_gtr = true,
                "set_in_datamaster" => args.set_in_datamaster = true,
                "timestamp" => {
                    input.parse::<syn::Token![=]>()?;
                    let value: usize = input.parse::<LitInt>()?.base10_parse()?;
                    args.timestamp = Some(value);
                }
                _ => {
                    return Err(syn::Error::new(ident.span(), "unknown argument"));
                }
            }

            let _ = input.parse::<syn::Token![,]>();
        }

        Ok(args)
    }
}

#[proc_macro_attribute]
pub fn init_gtr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = if attr.to_string() == "" || attr.to_string() == "()" {
        Args {
            init_gtr: false,
            set_in_datamaster: false,
            timestamp: None,
        }
    } else {
        syn::parse_macro_input!(attr as Args)
    };

    let func = syn::parse_macro_input!(item as ItemFn);
    let name = func.sig.ident.clone();

    let init_gtr = args.init_gtr;
    let set_in_datamaster = args.set_in_datamaster;
    let timestamp_expr = match args.timestamp {
        Some(v) => quote!(Some(#v)),
        None => quote!(None),
    };

    let expanded = quote! {
        #func

        inventory::submit! {
            engine::component::transformation::TransformationInit {
                gtr: #name,
                init_gtr: #init_gtr,
                set_in_datamaster: #set_in_datamaster,
                timestamp: #timestamp_expr,
            }
        }
    };

    //panic!("{}", expanded);

    expanded.into()
}
