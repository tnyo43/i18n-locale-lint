use std::error::Error;
use glob::glob;

pub fn get_files() -> Result<Vec<String>, Box<dyn Error>> {
    let pattern = "./example/*.json";
    let mut files = Vec::new();
    match glob(pattern) {
        Ok(paths) => {
            for path in paths {
                files.push(path.unwrap().to_str().unwrap().to_string());
            }
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }

    println!("{:?}", files);
    Ok(files)
}
