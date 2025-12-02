use globset::{Error, Glob};

use crate::File;

pub fn glob(files: Vec<File>, search: &str) -> Result<Vec<File>, Error> {
    let globber = Glob::new(search)?.compile_matcher();
    Ok(files
        .into_iter()
        .filter(|f| globber.is_match(f.path.clone()))
        .collect())
}
