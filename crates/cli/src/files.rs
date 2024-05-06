use std::{collections::HashMap, ffi::OsString};
use regex::Regex;
use crate::option;

fn group(file_paths: &Vec<OsString>, grouped_by: &str) -> HashMap<String, Vec<String>> {
    let re = Regex::new(grouped_by).unwrap();

    let mut grouped_paths = HashMap::new();

    for path in file_paths {
        let path = path.to_str().unwrap();
        if let Some(capture) = re.captures(path) {
            let key = capture.get(1).unwrap().as_str().to_string();
            grouped_paths
                .entry(key)
                .or_insert(Vec::new())
                .push(path.to_string());
        }
    }

    grouped_paths
}

pub fn get_file_groups() -> HashMap<String, Vec<String>> {
    let cli_option = option::INSTANCE.get().unwrap();
    let files = &cli_option.files;
    let grouped_by = match &cli_option.grouped_by {
        Some(grouped_by) => grouped_by,
        None => "^(.*/)([^/]+)$",
    };
    group(files, grouped_by)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, ffi::OsString};

    use crate::files::group;

    #[test]
    fn file_names_are_en_json_or_ja_json() {
        let files = [
            "./src/feature-x/user-list/i18n/en.json",
            "./src/feature-x/user-list/i18n/ja.json",
            "./src/feature-x/user-detail/i18n/en.json",
            "./src/feature-x/user-detail/i18n/ja.json",
            "./src/feature-y/user-list/i18n/en.json",
            "./src/feature-y/user-list/i18n/ja.json",
            "./src/deprecated/feature-x/user-list/i18n/en.json",
            "./src/deprecated/feature-x/user-list/i18n/ja.json",
        ]
        .iter()
        .map(|s| OsString::from(s.to_string()))
        .collect();

        let expected = HashMap::from([
            (
                "./src/feature-x/user-list/i18n/".to_string(),
                Vec::from([
                    "./src/feature-x/user-list/i18n/en.json".to_string(),
                    "./src/feature-x/user-list/i18n/ja.json".to_string(),
                ]),
            ),
            (
                "./src/feature-x/user-detail/i18n/".to_string(),
                Vec::from([
                    "./src/feature-x/user-detail/i18n/en.json".to_string(),
                    "./src/feature-x/user-detail/i18n/ja.json".to_string(),
                ]),
            ),
            (
                "./src/feature-y/user-list/i18n/".to_string(),
                Vec::from([
                    "./src/feature-y/user-list/i18n/en.json".to_string(),
                    "./src/feature-y/user-list/i18n/ja.json".to_string(),
                ]),
            ),
            (
                "./src/deprecated/feature-x/user-list/i18n/".to_string(),
                Vec::from([
                    "./src/deprecated/feature-x/user-list/i18n/en.json".to_string(),
                    "./src/deprecated/feature-x/user-list/i18n/ja.json".to_string(),
                ]),
            ),
        ]);
        assert_eq!(group(&files, "^(.*/)([^/]+)$"), expected);
    }

    #[test]
    fn file_names_are_xxx_en_json_or_xxx_ja_json() {
        let files = [
            "./src/feature-x/i18n/list.en.json",
            "./src/feature-x/i18n/list.ja.json",
            "./src/feature-x/i18n/detail.en.json",
            "./src/feature-x/i18n/detail.ja.json",
            "./src/feature-y/i18n/list.en.json",
            "./src/feature-y/i18n/list.ja.json",
            "./src/deprecated/feature-x/i18n/list.en.json",
            "./src/deprecated/feature-x/i18n/list.ja.json",
        ]
        .iter()
        .map(|s| OsString::from(s.to_string()))
        .collect();

        let expected = HashMap::from([
            (
                "./src/feature-x/i18n/list.".to_string(),
                Vec::from([
                    "./src/feature-x/i18n/list.en.json".to_string(),
                    "./src/feature-x/i18n/list.ja.json".to_string(),
                ]),
            ),
            (
                "./src/feature-x/i18n/detail.".to_string(),
                Vec::from([
                    "./src/feature-x/i18n/detail.en.json".to_string(),
                    "./src/feature-x/i18n/detail.ja.json".to_string(),
                ]),
            ),
            (
                "./src/feature-y/i18n/list.".to_string(),
                Vec::from([
                    "./src/feature-y/i18n/list.en.json".to_string(),
                    "./src/feature-y/i18n/list.ja.json".to_string(),
                ]),
            ),
            (
                "./src/deprecated/feature-x/i18n/list.".to_string(),
                Vec::from([
                    "./src/deprecated/feature-x/i18n/list.en.json".to_string(),
                    "./src/deprecated/feature-x/i18n/list.ja.json".to_string(),
                ]),
            ),
        ]);
        assert_eq!(group(&files, "^(.*)(en|ja).json$"), expected);
    }
}
