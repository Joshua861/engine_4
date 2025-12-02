use std::{
    fs::{self, read, ReadDir},
    io::{Cursor, Read},
};

use image::{DynamicImage, ImageReader};
use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenStream as Tk2};
use quote::quote;
use syn::{parse::Parse, parse2, Ident, Token};

#[derive(Debug)]
enum FileContents {
    Binary(Vec<u8>),
    Text(String),
    Image(DynamicImage),
    Json(serde_json::Value),
    Toml(toml::Value),
    Ron(ron::Value),
}

struct Input {
    path: String,
    const_name: String,
}

#[derive(Debug)]
enum Tree {
    Dir {
        name: String,
        contents: Vec<Tree>,
    },
    File {
        name: String,
        contents: FileContents,
        extention: String,
    },
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path: Literal = input.parse()?;
        let _ = input.parse::<Token![,]>();
        let name: Ident = input.parse()?;

        Ok(Self {
            path: path.to_string(),
            const_name: name.to_string(),
        })
    }
}

#[proc_macro]
pub fn include_folder(input: TokenStream) -> TokenStream {
    let ts: Tk2 = input.into();
    let input: Input = parse2(ts).unwrap_or_else(|e| panic!("Failed to parse input: {}", e));

    let dir = fs::read_dir(input.path).unwrap_or_else(|e| panic!("Failed to read root dir: {e}"));
    let contents = dir_contents(dir);
    dbg!(contents);

    quote! {}.into()
}

fn dir_contents(dir: ReadDir) -> Vec<Tree> {
    let mut contents = Vec::new();

    for c in dir {
        let c = c.expect("Could not read contents of dir");
        let ft = c.file_type().expect("Could not get file type");

        if ft.is_dir() {
            contents.push(Tree::Dir {
                name: c.file_name().into_string().expect("Failed to get dir name"),
                contents: dir_contents(fs::read_dir(c.path()).expect("Failed to get dir conents")),
            });
        } else {
            let f_contents = read(c.path()).expect("Failed to get file name");
            let extention = c
                .path()
                .extension()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or(String::new());
            let name = c
                .path()
                .file_name()
                .expect("Failed to get file name")
                .to_string_lossy()
                .to_string();

            match extention.to_lowercase().as_str() {
                "bmp" | "dds" | "ff" | "farbfeld" | "gif" | "hdr" | "ico" | "jpeg" | "jpg"
                | "exr" | "png" | "pnm" | "qoi" | "tga" | "tff" | "tiff" | "webp" => {
                    let image = ImageReader::new(Cursor::new(f_contents))
                        .with_guessed_format()
                        .expect("Failed to guess image format")
                        .decode()
                        .expect("Failed to decode image");

                    contents.push(Tree::File {
                        name,
                        contents: FileContents::Image(image),
                        extention,
                    });
                }
                "json" => {
                    let data: serde_json::Value = serde_json::from_str(
                        std::str::from_utf8(&f_contents).expect("Failed to decode utf8 in json"),
                    )
                    .expect("Failed to deserialize json");

                    contents.push(Tree::File {
                        name,
                        contents: FileContents::Json(data),
                        extention,
                    })
                }
                "ron" => {
                    let data: ron::Value = ron::from_str(
                        std::str::from_utf8(&f_contents).expect("Failed to decode utf8 in RON"),
                    )
                    .expect("Failed to deserialize RON");

                    contents.push(Tree::File {
                        name,
                        contents: FileContents::Ron(data),
                        extention,
                    })
                }
                "toml" => {
                    let data: toml::Value = toml::from_str(
                        std::str::from_utf8(&f_contents).expect("Failed to decode utf8 in TOML"),
                    )
                    .expect("Failed to deserialize TOML");

                    contents.push(Tree::File {
                        name,
                        contents: FileContents::Toml(data),
                        extention,
                    })
                }
                _ => {
                    if let Ok(s) = String::from_utf8(f_contents.clone()) {
                        contents.push(Tree::File {
                            name,
                            contents: FileContents::Text(s),
                            extention,
                        })
                    } else {
                        contents.push(Tree::File {
                            name,
                            contents: FileContents::Binary(f_contents),
                            extention,
                        })
                    }
                }
            }
        }
    }

    contents
}
