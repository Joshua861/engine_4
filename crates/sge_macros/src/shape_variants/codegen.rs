use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::parse::{Flag, ShapeEntry};

#[derive(Clone, Copy, PartialEq, Eq)]
enum DrawMode {
    Base,
    Outline,
    WithOutline,
}

impl DrawMode {
    fn suffix(self) -> &'static str {
        match self {
            DrawMode::Base => "",
            DrawMode::Outline => "_outline",
            DrawMode::WithOutline => "_with_outline",
        }
    }
}

#[derive(Clone, Copy)]
struct VariantSpec {
    rotation: bool,
    mode: DrawMode,
}

fn variants_for_flags(flags: &[Flag]) -> Vec<VariantSpec> {
    let has_rotation = flags.contains(&Flag::Rotation);
    let has_outline = flags.contains(&Flag::Outline);
    let has_with_outline = flags.contains(&Flag::WithOutline);

    let mut modes = vec![DrawMode::Base];
    if has_outline {
        modes.push(DrawMode::Outline);
    }
    if has_with_outline {
        modes.push(DrawMode::WithOutline);
    }

    let mut specs = Vec::new();
    for &mode in &modes {
        specs.push(VariantSpec {
            rotation: false,
            mode,
        });
        if has_rotation {
            specs.push(VariantSpec {
                rotation: true,
                mode,
            });
        }
    }
    specs
}

pub fn emit_entry(entry: &ShapeEntry) -> TokenStream {
    variants_for_flags(&entry.flags)
        .into_iter()
        .map(|spec| emit_variant(entry, spec))
        .collect()
}

fn emit_variant(entry: &ShapeEntry, spec: VariantSpec) -> TokenStream {
    let mode_suffix = spec.mode.suffix();
    let rot_suffix = if spec.rotation { "_rotation" } else { "" };
    let suffix = format!("{mode_suffix}{rot_suffix}");

    let fn_to = format_ident!("draw_{}{}_to", entry.name, suffix);
    let fn_screen = format_ident!("draw_{}{}", entry.name, suffix);
    let fn_world = format_ident!("draw_{}{}_world", entry.name, suffix);

    let is_outline_only = spec.mode == DrawMode::Outline;
    let filtered_params: Vec<&super::parse::Param> = entry
        .params
        .iter()
        .filter(|p| !(is_outline_only && p.name == "color"))
        .collect();

    let shape_decls: Vec<TokenStream> = filtered_params
        .iter()
        .map(|p| {
            let n = &p.name;
            let t = &p.ty;
            quote!(#n: #t,)
        })
        .collect();

    let extra_decls: Vec<TokenStream> = match spec.mode {
        DrawMode::Base => vec![],
        DrawMode::Outline => vec![quote!(thickness: f32,), quote!(outline_color: Color,)],
        DrawMode::WithOutline => vec![quote!(thickness: f32,), quote!(outline_color: Color,)],
    };

    let rot_decl: TokenStream = if spec.rotation {
        quote!(rot: f32,)
    } else {
        quote!()
    };

    let shape_args: Vec<&syn::Ident> = filtered_params.iter().map(|p| &p.name).collect();
    let extra_args: Vec<syn::Ident> = match spec.mode {
        DrawMode::Base => vec![],
        DrawMode::Outline | DrawMode::WithOutline => {
            vec![format_ident!("thickness"), format_ident!("outline_color")]
        }
    };
    let rot_arg = if spec.rotation {
        quote!(rot,)
    } else {
        quote!()
    };

    let forward = quote!(#(#shape_args,)* #(#extra_args,)* #rot_arg);

    let rot_binding = if !spec.rotation {
        quote!(let rot = 0.0_f32;)
    } else {
        quote!()
    };

    let ctor = &entry.constructor;

    let to_body = match spec.mode {
        DrawMode::Base => quote! {
            #rot_binding
            let __shape = { #ctor };
            __shape.draw_to(renderer);
        },
        DrawMode::Outline => quote! {
            #rot_binding
            let color = Color::TRANSPARENT;
            let __shape = { #ctor };
            __shape.draw_outline_to(renderer, thickness, outline_color);
        },
        DrawMode::WithOutline => quote! {
            #rot_binding
            let __shape = { #ctor };
            __shape.draw_with_outline_to(renderer, thickness, outline_color);
        },
    };

    let world_body = match spec.mode {
        DrawMode::Base => quote! {
            #rot_binding
            let __shape = { #ctor };
            __shape.draw_to(world_draw_queue_2d().renderer());
        },
        DrawMode::Outline => quote! {
            #rot_binding
            let color = Color::TRANSPARENT;
            let __shape = { #ctor };
            __shape.draw_outline_to(world_draw_queue_2d().renderer(), thickness, outline_color);
        },
        DrawMode::WithOutline => quote! {
            #rot_binding
            let __shape = { #ctor };
            __shape.draw_with_outline_to(world_draw_queue_2d().renderer(), thickness, outline_color);
        },
    };

    quote! {
        #[allow(clippy::too_many_arguments)]
        pub fn #fn_to(
            #(#shape_decls)*
            #(#extra_decls)*
            #rot_decl
            mut renderer: Renderer2D,
        ) {
            #to_body
        }

        #[allow(clippy::too_many_arguments)]
        pub fn #fn_screen(#(#shape_decls)* #(#extra_decls)* #rot_decl) {
            #fn_to(#forward draw_queue_2d().renderer());
        }

        #[allow(clippy::too_many_arguments)]
        pub fn #fn_world(#(#shape_decls)* #(#extra_decls)* #rot_decl) {
            #world_body
        }
    }
}
