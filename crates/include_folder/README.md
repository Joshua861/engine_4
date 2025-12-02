# Include Folder

A simple procedural macro to include the contents of a directory into your project.

This differs from [`include_dir`](https://crates.io/crates/include_dir), because it recursively generates structs for the directory and subdirectories making it easier to access files and so they show up in your IDE's autocomplete.

It first attempts to parse the contents of the file as UTF-8, if it can, it will store this as a
`String`, other wise it will store it as a `Vec<u8>`.

## Sanitization

Due to the fact that the files are stored as fields in a struct, the names have to follow
Rust's rules for identifier names. To make sure that the names are valid, the macro
will change the names if they would not be valid. It is not comprehensive but tries
to cover some common cases.

- If the name is just a keyword (e.g: `mod`), it will append an underscore (e.g: `for` => `for_`).
- If the name starts with a number it will replace that number with the word for that number.
  (e.g: `1Two3` => `OneTwo3`).
- If the name contains a `-`, it will be replaced with a `_`.

## Searching for files

### Globbing

Globbing can be done on any `directory` struct by using the `glob` method. The glob is
parsed by [`globset`](https://docs.rs/globset/latest/globset/), and are always recursive (i.e: not just top level files).

### all_X

For every file extention in a directory, an `all_[EXTENTION]` method will be generated.
This method returns a list of all the files with that extention. This is more efficient
than globbing because all the files are found at compile time, and no parsing is required.

> [!NOTE] For a file such as `index.min.js`, it's extention is treated as `js`.

## Example

Say we have a directory with a structure that looks like this:

```text
src
├── main.rs
├── nested
│   └── folders
│       └── test.txt
└── parsing
    ├── lexer.rs
    └── mod.rs
```

We can access the files in that directory like this:

```rs
use include_folder::include_folder;

// First argument is the path to the directory.
// Second argument is a name for that folder.
//   This crate uses heck under the hood so you can
//   use any case you want, for example: PascalCase.
// You can include multiple folders if you want.
include_folder!("./src", "src_dir");

fn main() {
    let dir = build_dir();
    let contents: String = dir.nested.folders.test.txt;

    dbg!(contents); // "Hello World!\n"

    // we can also get all the files in a folder
    dbg!(dir.files());

    // [src/main.rs:8:5] dir.files() = [
    //     File {
    //         path: "src.main.rs",
    //         data: Text(
    //             "",
    //         ),
    //     },
    //     File {
    //         path: "src.nested.folders.test.txt",
    //         data: Text(
    //             "Hello World!\n",
    //         ),
    //     },
    //      ...
    // ]
}
```

The generated code for this example looks like this (with all `impl` blocks removed):

```rs
use include_folder::include_folder;

struct BuildDir {
    nested: BuildDirNested,
    parsing: BuildDirParsing,
    main: BuildDirMain,
}
struct BuildDirNested {
    folders: BuildDirNestedFolders,
}
struct BuildDirNestedFolders {
    test: BuildDirNestedFoldersTest,
}
struct BuildDirNestedFoldersTest {
    txt: String,
}
struct BuildDirParsing {
    lexer: BuildDirParsingLexer,
}
struct BuildDirParsingLexer {
    rs: String,
}
struct BuildDirMain {
    rs: String,
}

fn build_dir() -> BuildDir {
    BuildDir {
        nested: BuildDirNested {
            folders: BuildDirNestedFolders {
                test: BuildDirNestedFoldersTest {
                    txt: "Hello World!\n".to_string(),
                },
            },
        },
        parsing: BuildDirParsing {
            lexer: BuildDirParsingLexer {
                rs: "".to_string(),
            },
        },
        main: BuildDirMain { rs: "".to_string() },
    }
}

fn main() {
    let dir = build_dir();
    let contents: String = dir.nested.folders.test.txt;
    // ...
}
```

This works also with multiple files of the same name. Say we added a `lexer.txt` next to the `lexer.rs`, we would get this instead:

```rs
struct BuildDirParsingLexer {
    txt: String,
    rs: String,
}
```

## Plans for the future

- Support for other file types.
  - Maybe using things like the `image` crate to automatically load and parse images at comp-time.
  - Main problem is making it so users can choose their own file types and how file of that file type are parsed.
