use crate::beatmap_types::Beatmap;
use regex::Regex;
use tokio::{
    fs::{self},
    io::{AsyncBufReadExt, BufReader},
};

pub async fn load(path: String) -> anyhow::Result<Vec<Beatmap>> {
    let file = fs::File::open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut bms = Vec::new();
    while let Some(line) = lines.next_line().await? {
        let re = Regex::new(r"(\d+)\s+(.+)\s+by:\s+(.+)").unwrap();
        if let Some(caps) = re.captures(&line) {
            let id = &caps[1];
            let name = &caps[2];
            let author = &caps[3];
            bms.push(Beatmap {
                id: id.to_string(),
                name: Some(name.into()),
                author: Some(author.into()),
            });
            // println!(
            //     "{} {} by: {}",
            //     style(id).blue(),
            //     style(name).red(),
            //     style(author).green()
            // );
        }
    }
    Ok(bms)
}
