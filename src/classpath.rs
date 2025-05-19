use std::path::{Path, PathBuf};
use std::env;
use crate::{manifest::Library, rules::is_all_rules_satisfied};

pub fn should_use_library(lib: &Library) -> bool {
    let rules_opt = &lib.rules;
    if !rules_opt.is_some() {
        return true;
    }

    let rules = rules_opt.as_ref().unwrap();
    return is_all_rules_satisfied(rules);
}

pub fn create_classpath(
    jar_file: PathBuf,
    libraries_path: PathBuf,
    libraries: Vec<Library>,
) -> String {
    let separator = if cfg!(windows) { ";" } else { ":" };
    let mut paths = vec![];

    for lib in libraries.iter() {
        if should_use_library(lib) {
            let artifact = &lib.downloads.artifact;
            let lib_path = &artifact.path;
            let fixed_lib_path = libraries_path.join(lib_path);
            paths.push(fixed_lib_path.to_str().unwrap().to_string());
        }
    }

    paths.push(jar_file.to_str().unwrap().to_string());

    paths.join(separator)
}


