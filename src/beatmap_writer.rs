// use cliux::{Boxed, Label, Tag};
use console::style;
use regex::Regex;
use std::path::Path;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

pub async fn import(path: &Path) -> anyhow::Result<()> {
    let mut entries = fs::read_dir(path).await?;
    let mut bms: Vec<String> = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name() {
                bms.push(name.to_string_lossy().into_owned());
            }
        }
    }
    let path = Path::new("./tsun.dere");
    export_file(path, &bms).await?;
    Ok(())
}

async fn export_file(path: &Path, bms: &[String]) -> anyhow::Result<()> {
    let mut file = File::create(path).await?;
    for bm in bms {
        let re = Regex::new(r"(\d+)(?:\s+(.+)\s+-\s+(.+))?").unwrap();
        if let Some(caps) = re.captures(bm) {
            let id = &caps[1];
            let author = caps.get(2).map(|m| m.as_str()).unwrap_or("Unknown");
            let name = caps.get(3).map(|m| m.as_str()).unwrap_or("Unknown");
            file.write_all(format!("{} {} by: {}", &id, &name, &author).as_bytes())
                .await?;
            file.write_all(b"\n").await?;
            println!(
                "{} {} by: {}",
                style(&id).blue(),
                style(&name).red(),
                style(&author).green()
            );
        }
    }
    println!(
        "{}\n{} {} {}",
        style("DONE").green().bold(),
        style("written").yellow(),
        style(bms.len()).blue(),
        style("Beatmaps").magenta(),
    );
    Ok(())
}
