use std::path::{Path, PathBuf};
use std::env;
use std::collections::HashMap;
use crate::{manifest::Library, rules::is_all_rules_satisfied};

pub fn should_use_library(lib: &Library) -> bool {
    let rules_opt = &lib.rules;
    if !rules_opt.is_some() {
        return true;
    }

    let rules = rules_opt.as_ref().unwrap();
    return is_all_rules_satisfied(rules);
}

pub fn filter_libraries_keep_newest(libs: Vec<Library>) -> Vec<Library> {
    let mut latest_versions: HashMap<String, Library> = HashMap::new();

    for lib in libs.into_iter() {
        let (base_name, version) = match lib.name.rsplit_once(':') {
            Some((n, v)) => (n.to_string(), v.to_string()),
            None => (lib.name.clone(), String::new()),
        };

        let version_nums: Vec<u32> = version.split('.').filter_map(|p| p.parse().ok()).collect();

        let update = match latest_versions.get(&base_name) {
            Some(existing) => {
                let existing_version = existing.name.rsplit_once(':').map(|(_, v)| v).unwrap_or("");
                let existing_nums: Vec<u32> = existing_version.split('.').filter_map(|p| p.parse().ok()).collect();
                version_nums > existing_nums
            }
            None => true,
        };

        if update {
            latest_versions.insert(base_name, lib);
        }
    }

    latest_versions.into_values().collect()
}

pub fn create_classpath(
    jar_file: PathBuf,
    libraries_path: PathBuf,
    libraries: Vec<Library>,
) -> String {
    let separator = if cfg!(windows) { ";" } else { ":" };
    let mut paths = vec![];

    let filtered_libs = filter_libraries_keep_newest(libraries);

    for lib in filtered_libs.iter() {
    //    if should_use_library(lib) {
            let artifact = &lib.downloads.artifact;
            let fixed_lib_path = libraries_path.join(&artifact.path);
            paths.push(fixed_lib_path.to_str().unwrap().to_string());
    //    }
    }

    paths.push(jar_file.to_str().unwrap().to_string());

    paths.join(separator)
}
