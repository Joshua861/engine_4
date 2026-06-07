use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Token, parse::Parse, parse_macro_input};

struct Input {
    ty: Ident,
    name: Ident,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ty = input.parse()?;
        input.parse::<Token![,]>()?;
        let name = input.parse()?;

        Ok(Self { ty, name })
    }
}

#[proc_macro]
pub fn global(input: TokenStream) -> TokenStream {
    let Input { ty, name } = parse_macro_input!(input as Input);

    let state = format!("{}_STATE", name.to_string().to_uppercase());
    let state = Ident::new(&state, name.span());

    let get = format!("get_{name}");
    let get = Ident::new(&get, name.span());

    let maybe_get = format!("maybe_get_{name}");
    let maybe_get = Ident::new(&maybe_get, name.span());

    let set = format!("set_{}", name);
    let set = Ident::new(&set, name.span());

    quote! {
        #[allow(static_mut_refs)]
        static mut #state: Option<#ty> = None;

        #[allow(static_mut_refs)]
        pub fn #get() -> &'static mut #ty {
            unsafe { #state.as_mut().unwrap_or_else(|| panic!()) }
        }

        #[allow(static_mut_refs)]
        pub fn #maybe_get() -> Option<&'static mut #ty> {
            unsafe { #state.as_mut() }
        }

        #[allow(static_mut_refs)]
        fn #set(value: #ty) {
            unsafe {
                #state = Some(value);
            }
        }
    }
    .into()
}
