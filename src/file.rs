pub fn read_path(path: &String) -> Result<std::io::BufReader<std::fs::File>, String> {
    if std::path::Path::new(path).exists() {
        match std::fs::File::open(path) {
            Ok(file) => Ok(std::io::BufReader::new(file)),
            Err(error) => Err(error.to_string()),
        }
    } else {
        #[cfg(feature = "url")]
        return match url::Url::parse(path) {
            Ok(url) => match &mut reqwest::blocking::get(url.to_owned()) {
                Ok(response) => {
                    let path = std::env::temp_dir().as_path().join("results.xml");
                    let mut file = std::fs::File::create(path.clone()).unwrap();
                    println!("{:?}", path.clone());
                    match response.copy_to(&mut file) {
                        Ok(_) => read_path(&String::from(path.to_str().unwrap())),
                        Err(error) => Err(error.to_string()),
                    }
                }
                Err(error) => Err(error.to_string()),
            },
            Err(error) => Err(error.to_string()),
        };

        #[cfg(not(feature = "url"))]
        return Err(format!("file not found: {path}"));
    }
}
