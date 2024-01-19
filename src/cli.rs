#[derive(Debug)]
pub enum CliOption {
    Directory(String),
}

impl CliOption {
    pub fn from_str(options_string_vec: &[String]) -> anyhow::Result<Vec<Self>> {
        let mut options_string_vec = options_string_vec.iter();
        let mut options: Vec<Self> = vec![];

        loop {
            let Some(key) = options_string_vec.next() else {
                break;
            };
            let Some(value) = options_string_vec.next() else {
                break;
            };
            let option = match key.as_str() {
                "--directory" => Self::Directory(value.into()),
                _ => anyhow::bail!("Unknown option {}", key),
            };
            options.push(option);
        }

        Ok(options)
    }
}
