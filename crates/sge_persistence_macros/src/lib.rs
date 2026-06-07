use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

fn is_lerpable(ty: &Type) -> bool {
    let lerpable = ["f32", "f64", "Vec2", "Vec3", "Vec4", "Color"];
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            return lerpable.contains(&seg.ident.to_string().as_str());
        }
    }
    false
}

fn has_arg(args: &TokenStream, name: &str) -> bool {
    if args.is_empty() {
        return false;
    }
    let args2: proc_macro2::TokenStream = args.clone().into();
    syn::parse::Parser::parse2(
        syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
        args2,
    )
    .map(|paths| paths.iter().any(|p| p.is_ident(name)))
    .unwrap_or(false)
}

#[proc_macro_attribute]
pub fn persistent(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;
    let generics = &input.generics;

    let generate_diff = has_arg(&args, "diff");
    let generate_lerp = has_arg(&args, "lerp");

    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => f.named.iter().cloned().collect::<Vec<_>>(),
            _ => {
                return syn::Error::new_spanned(
                    name,
                    "#[persistent] only supports named-field structs",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(name, "#[persistent] only supports structs")
                .to_compile_error()
                .into();
        }
    };

    let struct_fields = fields.iter().map(|f| {
        let fname = &f.ident;
        let fty = &f.ty;
        let fattrs = f.attrs.iter().filter(|a| !a.path().is_ident("delta"));
        quote! { #(#fattrs)* #fname: #fty }
    });

    let lerp_code = if generate_lerp {
        let lerp_arms = fields.iter().map(|f| {
            let fname = &f.ident;
            let fty = &f.ty;
            if is_lerpable(fty) {
                quote! {
                    #fname: ::sge_persistence::PartialLerp::partial_lerp(&self.#fname, &other.#fname, t)
                }
            } else {
                quote! {
                    #fname: self.#fname.clone()
                }
            }
        });

        quote! {
            impl #generics ::sge_persistence::PartialLerp for #name #generics {
                fn partial_lerp(&self, other: &Self, t: f32) -> Self {
                    Self {
                        #(#lerp_arms,)*
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let diff_code = if generate_diff {
        let delta_name = quote::format_ident!("{}Diff", name);

        let is_sparse = |f: &syn::Field| {
            f.attrs.iter().any(|a| {
                if !a.path().is_ident("delta") {
                    return false;
                }
                matches!(a.parse_args::<syn::Path>(), Ok(p) if p.is_ident("sparse"))
            })
        };

        let delta_fields = fields.iter().map(|f| {
            let fname = &f.ident;
            let fty = &f.ty;
            if is_sparse(f) {
                quote! { #fname: Option<::std::vec::Vec<(usize, u8)>> }
            } else {
                quote! { #fname: Option<#fty> }
            }
        });

        let diff_arms = fields.iter().map(|f| {
            let fname = &f.ident;
            if is_sparse(f) {
                quote! {
                    #fname: {
                        let changes: ::std::vec::Vec<(usize, u8)> = self.#fname
                            .iter()
                            .zip(old.#fname.iter())
                            .enumerate()
                            .filter_map(|(i, (a, b))| if a != b { Some((i, *a)) } else { None })
                            .collect();
                        if changes.is_empty() { None } else { Some(changes) }
                    }
                }
            } else {
                quote! {
                    #fname: (self.#fname != old.#fname).then(|| self.#fname.clone())
                }
            }
        });

        let apply_arms = fields.iter().map(|f| {
            let fname = &f.ident;
            if is_sparse(f) {
                quote! {
                    if let Some(changes) = &diff.#fname {
                        for &(i, v) in changes { self.#fname[i] = v; }
                    }
                }
            } else {
                quote! {
                    if let Some(v) = diff.#fname { self.#fname = v; }
                }
            }
        });

        let has_changes_arms = fields.iter().map(|f| {
            let fname = &f.ident;
            quote! { self.#fname.is_some() }
        });

        quote! {
            #[derive(
                ::sge_persistence::rkyv::Archive,
                ::sge_persistence::rkyv::Serialize,
                ::sge_persistence::rkyv::Deserialize,
                Debug,
            )]
            #vis struct #delta_name #generics { #(#delta_fields,)* }

            impl #generics ::sge_persistence::Persistent for #generics #delta_name #generics {
                fn to_bytes(&self) -> ::sge_persistence::Result<::std::vec::Vec<u8>> {
                    use ::sge_persistence::rkyv::ser::Serializer as _;
                    let bytes = ::sge_persistence::rkyv::to_bytes::<::sge_persistence::rkyv::rancor::Error>(self)?;
                    Ok(bytes.into_vec())
                }
                fn from_bytes(bytes: impl ::core::convert::AsRef<[u8]>) -> ::sge_persistence::Result<Self> {
                    let archived = ::sge_persistence::rkyv::access::<<Self as ::sge_persistence::rkyv::Archive>::Archived, ::sge_persistence::rkyv::rancor::Error>(bytes.as_ref())?;
                    Ok(::sge_persistence::rkyv::deserialize::<Self, ::sge_persistence::rkyv::rancor::Error>(archived).unwrap())
                }
            }

            impl #generics ::sge_persistence::Diff for #delta_name #generics {
                type Data = #name;

                fn has_changes(&self) -> bool {
                    #(#has_changes_arms)||*
                }
            }

            impl #generics ::sge_persistence::Diffable for #name #generics {
                type Diff = #delta_name #generics;

                fn diff(&self, old: &Self) -> Self::Diff {
                    #delta_name { #(#diff_arms,)* }
                }

                fn apply_diff(&mut self, diff: Self::Diff) {
                    #(#apply_arms)*
                }
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #(#attrs)*
        #[derive(
            ::sge_persistence::rkyv::Archive,
            ::sge_persistence::rkyv::Serialize,
            ::sge_persistence::rkyv::Deserialize,
        )]
        #vis struct #name #generics { #(#struct_fields,)* }

        impl #generics ::sge_persistence::Persistent for #generics #name #generics {
            fn to_bytes(&self) -> ::sge_persistence::Result<Vec<u8>> {
                use ::sge_persistence::rkyv::ser::Serializer as _;
                let bytes = ::sge_persistence::rkyv::to_bytes::<::sge_persistence::rkyv::rancor::Error>(self)?;
                Ok(bytes.into_vec())
            }
            fn from_bytes(bytes: impl AsRef<[u8]>) -> ::sge_persistence::Result<Self> {
                let archived = ::sge_persistence::rkyv::access::<<Self as ::sge_persistence::rkyv::Archive>::Archived, ::sge_persistence::rkyv::rancor::Error>(bytes.as_ref())?;
                Ok(::sge_persistence::rkyv::deserialize::<Self, ::sge_persistence::rkyv::rancor::Error>(archived).unwrap())
            }
        }

        #lerp_code
        #diff_code
    };

    TokenStream::from(expanded)
}
