use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct Completion {
    pub regexp_left: String,
    pub regexp_right: String,
    pub command: String,
    pub fzf_command: String,
}

#[derive(Debug, Deserialize)]
pub struct Snippet {
    pub name: String,
    pub keyword: String,
    pub snippet: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub completions: Vec<Completion>,
    pub snippets: Vec<Snippet>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let home_dir = match dirs::home_dir() {
            Some(path) => path,
            None => {
                println!("Home directory not found");
                std::process::exit(1);
            }
        };

        let setting_file = format!(
            "{}/.config/fzf-preview.zsh/config.yml",
            home_dir.to_str().unwrap()
        );

        let mut settings = Config::new();
        settings
            .merge(File::with_name(&setting_file).required(false))
            .unwrap();

        settings.try_into()
    }
}
