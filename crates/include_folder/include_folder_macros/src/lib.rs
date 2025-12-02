use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use heck::{ToPascalCase, ToSnekCase};
use include_folder_shared::{File, FileData};
// use image::DynamicImage;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    LitStr, Token,
};

#[derive(Debug, Clone)]
enum Tree {
    Leaf(FileData),
    Branch(HashMap<String, Tree>),
}

struct Input {
    path: String,
    name: String,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse::<LitStr>()?.value();

        let _ = input.parse::<Token![,]>()?;

        let name = input.parse::<LitStr>()?.value();

        Ok(Input { path, name })
    }
}

#[proc_macro]
pub fn include_folder(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as Input);

    let tree = match build_tree(&input.path) {
        Ok(tree) => tree,
        Err(e) => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Failed to build directory tree: {}", e),
            )
            .to_compile_error()
            .into();
        }
    };

    let proc_tree = process_tree(tree.clone());

    let mut top = quote! {};
    let function_name = syn::Ident::new(
        &to_snek_name(input.name.clone()),
        proc_macro2::Span::call_site(),
    );
    let struct_name = to_struct_name(input.name);
    let inner = gen_structs(proc_tree, &mut top, struct_name.clone());
    let return_type = syn::Ident::new(&struct_name, proc_macro2::Span::call_site());
    let impls = gen_impls(tree, struct_name);

    let output = quote! {
        #top

        #impls

        fn #function_name () -> #return_type {
            #inner
        }
    };

    // DEBUGGING ONLY
    // fs::write("/tmp/tokens.rs", output.to_string());

    output.into()
}

fn build_tree(dir_path: &str) -> Result<Tree> {
    let path = Path::new(dir_path);
    if !path.exists() {
        return Err(anyhow::anyhow!("Path does not exist: {}", dir_path));
    }

    if path.is_file() {
        let file = read_file(path)?;
        return Ok(Tree::Leaf(file));
    }

    let mut dir_map = HashMap::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(file_name) = path.file_name() {
            if file_name.to_string_lossy().starts_with('.') {
                continue;
            }
        }

        let file_name = path.file_name().unwrap().to_string_lossy().to_string();

        if path.is_file() {
            match read_file(&path) {
                Ok(file) => {
                    dir_map.insert(file_name, Tree::Leaf(file));
                }
                Err(e) => {
                    eprintln!("Error reading file {}: {}", path.display(), e);
                    continue;
                }
            }
        } else if path.is_dir() {
            match build_tree(path.to_str().unwrap()) {
                Ok(branch) => {
                    dir_map.insert(file_name, branch);
                }
                Err(e) => {
                    eprintln!("Error processing directory {}: {}", path.display(), e);
                    continue;
                }
            }
        }
    }

    Ok(Tree::Branch(dir_map))
}

fn read_file(path: &Path) -> Result<FileData> {
    /*let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    if extension == "png" || extension == "jpg" || extension == "jpeg" || extension == "gif" {
        let img = image::open(path)?;
        Ok(FileData::Image(img))
    } else*/
    {
        let content = fs::read(path)?;

        match String::from_utf8(content.clone()) {
            Ok(text) => Ok(FileData::Text(text)),
            Err(_) => Ok(FileData::Blob(content)),
        }
    }
}

fn process_tree(tree: Tree) -> Tree {
    match tree {
        Tree::Branch(map) => {
            let mut new_map = HashMap::new();

            for (k, v) in map {
                match v {
                    Tree::Leaf(file) => {
                        let parts: Vec<String> = k
                            .split('.')
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect();

                        if parts.is_empty() {
                            new_map.insert(k, Tree::Leaf(file));
                        } else {
                            merge_into_map(&mut new_map, &parts, file);
                        }
                    }
                    Tree::Branch(inner_map) => {
                        new_map.insert(k, process_tree(Tree::Branch(inner_map)));
                    }
                }
            }

            Tree::Branch(new_map)
        }
        Tree::Leaf(file) => Tree::Leaf(file),
    }
}

fn merge_into_map(map: &mut HashMap<String, Tree>, parts: &[String], file: FileData) {
    if parts.is_empty() {
        return;
    }

    let current_part = &parts[0];

    if parts.len() == 1 {
        map.insert(current_part.clone(), Tree::Leaf(file));
        return;
    }

    if !map.contains_key(current_part) {
        map.insert(current_part.clone(), Tree::Branch(HashMap::new()));
    }

    if let Some(Tree::Branch(ref mut next_map)) = map.get_mut(current_part) {
        merge_into_map(next_map, &parts[1..], file);
    } else {
        let mut next_map = HashMap::new();
        merge_into_map(&mut next_map, &parts[1..], file);
        map.insert(current_part.clone(), Tree::Branch(next_map));
    }
}

fn gen_impls(tree: Tree, path: String) -> TokenStream2 {
    let path_ident = syn::Ident::new(
        &to_struct_name(path.clone()),
        proc_macro2::Span::call_site(),
    );

    match tree {
        Tree::Leaf(_data) => {
            quote! {}
        }
        Tree::Branch(map) => {
            let files = get_files(Tree::Branch(map.clone()), "".to_string());

            let dir_impl = {
                let files = files.iter().map(|f| {
                    let dotted = f.path.replace('/', ".");
                    let parts = &dotted.split('.');
                    let path = &f.path;
                    let mut tokens = quote! { self };

                    for part in parts.clone() {
                        let ident = syn::Ident::new(&sanitisze_name(part.to_string()), proc_macro2::Span::call_site());
                        tokens = quote! { #tokens.#ident };
                    }

                    quote! { ::include_folder::File {path: #path.to_string(), data: #tokens.clone().to_file_data() } }
                });

                let files = quote! { vec![ #(#files),* ] };

                let tokens = quote! {
                    impl ::include_folder::Directory for #path_ident {
                        fn files(&self) -> Vec<::include_folder::File> {
                            use ::include_folder::Data;

                            #files
                        }
                    }
                };

                tokens
            };

            let extention_impls = {
                let mut functions = quote! {};
                let mut extentions = files
                    .iter()
                    .filter_map(|f| Path::new(&f.path).extension().and_then(|os| os.to_str()))
                    .collect::<Vec<_>>();
                extentions.sort();
                extentions.dedup();

                for extention in extentions {
                    let ident = syn::Ident::new(
                        &sanitisze_name(format!("all_{}", extention)),
                        proc_macro2::Span::call_site(),
                    );
                    let files = files.iter().filter(|f| {
                        Path::new(&f.path)
                            .extension()
                            .and_then(|os| os.to_str())
                            .is_some_and(|e| e == extention)
                    });

                    let array = files.map(|f| {
                        let mut tokens = quote! { self };
                        let dotted = f.path.replace('/', ".");
                        let parts = &dotted.split('.');
                        let path = &f.path;

                        for part in parts.clone() {
                            let ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                            tokens = quote! { #tokens.#ident };
                        }

                        quote! { ::include_folder::File {path: #path.to_string(), data: #tokens.clone().to_file_data() } }
                    });

                    let function = quote! {
                        fn #ident(&self) -> Vec<::include_folder::File> {
                            use ::include_folder::Data;
                            vec![ #(#array),* ]
                        }
                    };

                    functions = quote! {
                        #functions
                        #function
                    }
                }

                quote! {
                    impl #path_ident {
                        #functions
                    }
                }
            };

            let mut tokens = quote! {
                #dir_impl

                #extention_impls
            };

            for (k, v) in map {
                let nested_path = format!("{}{}", path, to_struct_name(k));
                let added_tokens = gen_impls(v, nested_path);

                tokens = quote! {
                    #tokens

                    #added_tokens
                }
            }

            tokens
        }
    }
}

fn gen_structs(proc_tree: Tree, top: &mut TokenStream2, path: String) -> TokenStream2 {
    let path_ident = syn::Ident::new(
        &to_struct_name(path.clone()),
        proc_macro2::Span::call_site(),
    );

    match proc_tree {
        Tree::Leaf(file) => match file {
            FileData::Blob(data) => {
                let iter = data.into_iter();
                quote! { vec![ #(#iter),* ] }
            }
            FileData::Text(text) => {
                quote! { #text.to_string() }
            }
        },
        Tree::Branch(map) => {
            let types = map.iter().map(|(key, value)| {
                let key_ident =
                    syn::Ident::new(&sanitisze_name(key.clone()), proc_macro2::Span::call_site());

                let type_path = match value {
                    Tree::Branch(_) => format!("{}{}", path, to_struct_name(key.clone())),
                    Tree::Leaf(file) => file._type(),
                };

                let type_ident = syn::Ident::new(&type_path, proc_macro2::Span::call_site());

                quote! {
                    #key_ident: #type_ident
                }
            });

            let struct_declaration = quote! {
                #[derive(Clone, Debug)]
                struct #path_ident {
                    #(#types),*
                }
            };

            *top = quote! {
                #top
                #struct_declaration
            };

            let fields = map.into_iter().map(|(key, value)| {
                let key_ident =
                    syn::Ident::new(&sanitisze_name(key.clone()), proc_macro2::Span::call_site());
                let nested_path = format!("{}{}", path, to_struct_name(key));
                let value = gen_structs(value, top, nested_path);

                quote! {
                    #key_ident: #value
                }
            });

            quote! {
                #path_ident {
                    #(#fields),*
                }
            }
        }
    }
}

fn get_files(tree: Tree, path: String) -> Vec<File> {
    match tree {
        Tree::Leaf(data) => vec![File { path, data }],
        Tree::Branch(map) => {
            let mut files = vec![];
            for (key, value) in map {
                let path = if path.is_empty() {
                    key
                } else {
                    format!("{}/{}", path, key)
                };
                for file in get_files(value, path) {
                    files.push(file);
                }
            }

            files
        }
    }
}

fn sanitisze_name(string: String) -> String {
    if string.is_empty() {
        panic!("Identifier name was empty.");
    }

    let string = string.replace("-", "_");

    let reserved_words = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "priv", "pub",
        "ref", "return", "self", "static", "struct", "super", "trait", "true", "try", "type",
        "use", "where", "while", "yield", "async", "await", "dyn", "abstract", "become", "box",
        "do", "final", "macro", "override", "priv", "try", "unsized", "virtual", "where", "gen",
        "typeof",
    ];

    if reserved_words.contains(&string.as_str()) {
        return format!("{}_", string);
    }

    let numbers = [
        ('0', "Zero"),
        ('1', "One"),
        ('2', "Two"),
        ('3', "Three"),
        ('4', "Four"),
        ('5', "Five"),
        ('6', "Six"),
        ('7', "Seven"),
        ('8', "Eight"),
        ('9', "Nine"),
    ];

    let first_char = string.chars().next().unwrap();

    for (find, replace) in numbers {
        if first_char == find {
            return format!("{}{}", replace, string.chars().skip(1).collect::<String>());
        }
    }

    string
}

fn to_struct_name(string: String) -> String {
    sanitisze_name(string).to_pascal_case()
}

fn to_snek_name(string: String) -> String {
    sanitisze_name(string).to_snek_case()
}

// fn get_files(tree: Tree, path: String) -> Vec<File> {
//     match tree {
//         Tree::Leaf(data) => vec![File { path, data }],
//         Tree::Branch(map) => {
//             let mut files = vec![];
//             for (key, value) in map {
//                 let path = if path.is_empty() {
//                     key
//                 } else {
//                     format!("{}.{}", path, key)
//                 };
//                 for file in get_files(value, path) {
//                     files.push(file);
//                 }
//             }
//
//             files
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_valid() {
        for string in ["foo", "Bar", "Bar0", "FooFoo_9_", "for_bar", "modd"] {
            assert_eq!(sanitisze_name(string.to_string()), string);
        }
    }

    #[test]
    fn sanitize_keywords() {
        for (input, output) in [
            ("mod", "mod_"),
            ("for", "for_"),
            ("fn", "fn_"),
            ("else", "else_"),
        ] {
            assert_eq!(sanitisze_name(input.to_string()), output);
        }
    }

    #[test]
    fn sanitize_number_start() {
        for (input, output) in [
            ("0", "Zero"),
            ("1", "One"),
            ("2", "Two"),
            ("3", "Three"),
            ("1Two3", "OneTwo3"),
        ] {
            assert_eq!(sanitisze_name(input.to_string()), output);
        }
    }

    #[test]
    fn sanitize_dashes() {
        for (input, output) in [
            ("hello-world", "hello_world"),
            ("hello-world-2", "hello_world_2"),
            ("test_", "test_"),
            ("test--", "test__"),
        ] {
            assert_eq!(sanitisze_name(input.to_string()), output);
        }
    }
}
