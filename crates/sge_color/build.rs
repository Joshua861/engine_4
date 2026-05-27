use std::fs;
use std::path::Path;

use heck::ToShoutySnakeCase;
use palette::{IntoColor, Oklch, Srgb};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

fn oklch_to_srgb(l: f32, c: f32, h: f32) -> (f32, f32, f32) {
    let oklch = Oklch::new(l, c, h);
    let srgb: Srgb = oklch.into_color();
    (srgb.red, srgb.green, srgb.blue)
}

fn main() {
    println!("cargo:rerun-if-changed=colors.json");
    println!("cargo:rerun-if-changed=build.rs");

    if Path::new("src/data.rs").exists() {
        return;
    }

    let colors_json = fs::read_to_string("colors.json").expect("failed to read colors.json");
    let data: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(&colors_json).expect("failed to parse colors.json");

    let mut color_impl_items = TokenStream::new();
    let mut palette_impl_items = TokenStream::new();
    let mut color_map_entries = TokenStream::new();

    let mut color_const_idents: Vec<proc_macro2::Ident> = Vec::new();
    let mut palette_const_idents: Vec<proc_macro2::Ident> = Vec::new();

    color_impl_items.extend(quote! {
        pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    });

    color_map_entries.extend(quote! {
        "TRANSPARENT" => Color::TRANSPARENT,
    });

    let css_json = fs::read_to_string("css.json").expect("failed to read colors.json");
    let css_data: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(&css_json).expect("failed to parse css.json");

    for (color_name, hex) in css_data {
        let name = color_name.to_shouty_snake_case();
        let name_ident = Ident::new(&name, Span::call_site());
        let name = name.replace("_", "");
        let hex = hex.to_string();
        let hex_int = hex.strip_prefix("\"#").unwrap().strip_suffix('\"').unwrap();
        let hex_int = syn::LitInt::new(&format!("0x{}", hex_int), Span::call_site());

        color_map_entries.extend(quote! {
            #name => Color::#name_ident,
        });

        color_impl_items.extend(quote! {
            pub const #name_ident: Self = Self::hex(#hex_int);
        });
    }

    for (color_name, brightnesses) in &data {
        let brightnesses = brightnesses.as_object().expect("expected object");
        let palette_upper = format_ident!("{}", color_name.to_uppercase());
        palette_const_idents.push(palette_upper.clone());

        let mut palette_fields = TokenStream::new();

        for brightness in [
            "50", "100", "200", "300", "400", "500", "600", "700", "800", "900", "950",
        ] {
            let const_name = format_ident!("{}_{}", color_name.to_uppercase(), brightness);
            color_const_idents.push(const_name.clone());

            let oklch_arr = brightnesses
                .get(brightness)
                .expect("missing brightness")
                .as_array()
                .expect("expected array");

            let l = oklch_arr[0].as_f64().unwrap() as f32 / 100.0;
            let c = oklch_arr[1].as_f64().unwrap() as f32;
            let h = oklch_arr[2].as_f64().unwrap() as f32;

            let (r, g, b) = oklch_to_srgb(l, c, h);

            color_impl_items.extend(quote! {
                pub const #const_name: Self = Self::new(#r, #g, #b);
            });

            let field_name = format_ident!("v{}", brightness);
            palette_fields.extend(quote! {
                #field_name: Color::#const_name,
            });

            let name_str = format!("{}_{}", color_name.to_uppercase(), brightness);

            let norm = {
                let raw = format!(
                    "{}{}",
                    color_name.to_uppercase().replace("_", ""),
                    brightness
                );
                let raw = raw.replace("00", "");

                if raw.ends_with("950") {
                    raw[..raw.len() - 3].to_string() + "9.5"
                } else if raw.ends_with("50") && !raw.ends_with("950") {
                    raw[..raw.len() - 2].to_string() + "0.5"
                } else {
                    raw
                }
            };

            color_map_entries.extend(quote! {
                #name_str => Color::#const_name,
                #norm => Color::#const_name,
            });

            if norm.contains('.') {
                let norm2 = norm.replace('.', "");
                color_map_entries.extend(quote! {
                    #norm2 => Color::#const_name,
                });
            }
        }

        palette_impl_items.extend(quote! {
            pub const #palette_upper: Self = Self {
                #palette_fields
            };
        });
    }

    let color_count = color_const_idents.len();
    let palette_count = palette_const_idents.len();

    let tokens = quote! {
        use super::*;

        impl Color {
            #color_impl_items

            pub const ALL: [Self; #color_count] = [#(Self::#color_const_idents),*];
        }

        impl Palette {
            #palette_impl_items

            pub const PALETTES: [Self; #palette_count] = [#(Self::#palette_const_idents),*];
        }

        use phf::phf_map;

        pub static COLOR_MAP: phf::Map<&'static str, Color> = phf_map! {
            #color_map_entries
        };
    };

    let syntax_tree = syn::parse2(tokens).expect("failed to parse generated tokens");
    let formatted = prettyplease::unparse(&syntax_tree);

    let out_path = Path::new("src/data.rs");
    fs::write(out_path, formatted).expect("failed to write src/data.rs");
}
