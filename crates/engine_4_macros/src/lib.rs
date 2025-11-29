use proc_macro::TokenStream;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use syn::parse::Parse;
use syn::{Expr, Ident, Token, Visibility, parse_macro_input};

struct Actions {
    actions: Vec<Action>,
}

impl Parse for Actions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut actions: Vec<Action> = vec![];

        while !input.is_empty() {
            if let Some(action) = try_parse_action(input) {
                actions.push(action);
                let _ = input.parse::<Token![,]>();
            } else {
                break;
            }
        }

        Ok(Self { actions })
    }
}

impl Actions {
    fn to_tokens(self) -> Ts2 {
        let mut ts = Ts2::new();

        for (i, action) in self.actions.iter().enumerate() {
            let action = action.to_tokens(i as u32);
            ts = quote! { #ts #action };
        }

        ts
    }
}

impl Action {
    fn to_tokens(&self, n: u32) -> Ts2 {
        let Action { name, visibility } = self;
        quote! { #visibility const #name: ::engine_4::prelude::Action = ::engine_4::prelude::Action::new(#n); }
    }
}

fn try_parse_action(input: syn::parse::ParseStream) -> Option<Action> {
    let visibility;
    if let Ok(v) = input.parse::<Visibility>() {
        visibility = v;
    } else {
        visibility = Visibility::Inherited;
    }

    let name: Ident = input.parse().ok()?;
    Some(Action { name, visibility })
}

struct Action {
    name: Ident,
    visibility: Visibility,
}

#[proc_macro]
pub fn actions(input: TokenStream) -> TokenStream {
    let actions = parse_macro_input!(input as Actions);
    actions.to_tokens().into()
}

struct Binds {
    binds: Vec<(Expr, Expr)>,
}

impl Parse for Binds {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut binds = vec![];

        loop {
            if let Ok(name) = input.parse() {
                input.parse::<Token![=>]>()?;
                if let Ok(value) = input.parse() {
                    input.parse::<Token![;]>()?;
                    binds.push((name, value))
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(Self { binds })
    }
}

#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
    let binds = parse_macro_input!(input as Binds);

    let mut tokens = quote! {};

    for bind in binds.binds {
        let (name, value) = bind;
        tokens = quote! {
            #tokens

            ::engine_4::prelude::bind_button(#name, #value.into());
        };
    }

    tokens.into()
}
