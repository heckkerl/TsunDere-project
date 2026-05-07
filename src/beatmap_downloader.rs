use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use rquest_util::Emulation;
use std::time::Duration;
use tokio::{io::AsyncWriteExt, time::sleep};

use crate::beatmap_types::Beatmap;

async fn downloader(url: String) -> anyhow::Result<()> {
    let file_name = &url.split("beatmapsets/").nth(1).unwrap().split("/").nth(0);
    let output = format!("maps/{}.osz", file_name.expect("ERROR"));
    tokio::fs::create_dir_all("./maps").await?;
    let mut output_file = tokio::fs::File::create(&output).await?;
    let client = rquest::Client::builder()
        .emulation(Emulation::Chrome131)
        .redirect(rquest::redirect::Policy::limited(5))
        .build()?;
    let resp = client.get(&url).send().await?;
    let bytes_resp = &resp.bytes().await?;
    output_file.write_all(&bytes_resp).await?;
    Ok(())
}

pub async fn batch_download(maps: &[Beatmap], delay: u64) -> anyhow::Result<()> {
    let bar = ProgressBar::new(maps.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.blue} Downloading [{bar:40.magenta}] {pos}/{len} [{elapsed_precise}]",
            )
            .unwrap()
            .progress_chars("=> "),
    );
    for map in maps {
        bar.tick();
        let id = &map.id;
        let url = format!("https://osu.ppy.sh/beatmapsets/{}/download", &id);
        let name = map.name.clone().unwrap_or(String::from("None"));
        let author = map.name.clone().unwrap_or(String::from("None"));
        bar.tick();
        bar.println(format!(
            "{} {} {} by: {}",
            style("downloading").yellow(),
            style(id).blue(),
            style(&name).red(),
            style(author).green()
        ));
        bar.tick();
        match downloader(url).await {
            Ok(()) => {}
            Err(err) => {
                bar.println(format!("{}", err));
            }
        }
        bar.inc(1);
        bar.tick();
        sleep(Duration::from_millis(delay)).await;
        bar.tick();
    }
    bar.finish_and_clear();

    println!(
        "{} {} {}",
        style("Downloaded").green(),
        style(maps.len()).blue(),
        style("Beatmaps").magenta().dim()
    );
    Ok(())
}
