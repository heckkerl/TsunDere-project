use console::style;
#[derive(Clone)]
pub struct Beatmap {
    pub id: String,
    pub name: Option<String>,
    pub author: Option<String>,
}

impl From<String> for Beatmap {
    fn from(url: String) -> Self {
        let re = regex::Regex::new(r"beatmapsets/(\d+)/download").unwrap();
        match re.captures(url.as_str()) {
            Some(id) => Self {
                id: id[1].to_string(),
                name: None,
                author: None,
            },
            None => Self {
                id: "0".into(),
                name: None,
                author: None,
            },
        }
    }
}

impl std::fmt::Debug for Beatmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} - {}\n",
            style(&self.id).blue(),
            style(self.author.as_ref().unwrap_or(&"None".to_string())).green(),
            style(self.name.as_ref().unwrap_or(&"None".to_string())).red(),
        )
    }
}
