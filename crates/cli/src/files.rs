use std::collections::HashMap;
use regex::Regex;
use crate::option;

fn group(file_paths: &Vec<String>, grouped_by: &str) -> HashMap<String, Vec<String>> {
    let re = Regex::new(grouped_by).unwrap();

    let mut grouped_paths = HashMap::new();

    for paths in file_paths {
        if let Some(capture) = re.captures(paths) {
            let key = capture.get(1).unwrap().as_str().to_string();
            grouped_paths
                .entry(key)
                .or_insert(Vec::new())
                .push(paths.to_string());
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
    use std::collections::HashMap;

    use crate::files::group;

    #[test]
    fn test_case() {
        let files = vec![
            "./src/feature-x/user-list/i18n/en.json".to_string(),
            "./src/feature-x/user-list/i18n/ja.json".to_string(),
            "./src/feature-x/user-detail/i18n/en.json".to_string(),
            "./src/feature-x/user-detail/i18n/ja.json".to_string(),
            "./src/feature-y/user-list/i18n/en.json".to_string(),
            "./src/feature-y/user-list/i18n/ja.json".to_string(),
            "./src/deprecated/feature-x/user-list/i18n/en.json".to_string(),
            "./src/deprecated/feature-x/user-list/i18n/ja.json".to_string(),
        ];

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
}
