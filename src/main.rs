use std::{
    fs,
    net::{IpAddr, Ipv4Addr},
    path::Path,
    thread,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Local};
use hotwatch::{Event, Hotwatch};
use warp::Filter;

mod templates;

const CONTENT: &str = "content";
const PUBLIC: &str = "public";

type SSRGResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> SSRGResult<()> {
    rebuild_site(CONTENT, PUBLIC)?;

    tokio::task::spawn_blocking(move || {
        let mut watch = Hotwatch::new().expect("hotwatch failed");

        watch
            .watch(CONTENT, |_event: Event| {
                rebuild_site(CONTENT, PUBLIC).expect("rebuilding");
            })
            .expect("failed to watch content");

        loop {
            thread::sleep(Duration::from_secs(1));
        }
    });

    let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let index = warp::get().and(warp::fs::dir(PUBLIC));

    warp::serve(index).run((addr, 4000)).await;
    Ok(())
}

fn rebuild_site(content_dir: &str, output_dir: &str) -> SSRGResult<()> {
    let _ = fs::remove_dir_all(output_dir);

    let mut markdown_files: Vec<(String, SystemTime)> = walkdir::WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().display().to_string().ends_with(".md"))
        .map(|e| {
            (
                e.path().display().to_string(),
                e.metadata()
                    .expect("failed to get metadata")
                    .modified()
                    .expect("failed to get modified")
            )
        })
        .collect();

    markdown_files.sort_by_key(|file| file.1);
    markdown_files.reverse();

    let mut html_files = Vec::with_capacity(markdown_files.len());

    for file in &markdown_files {
        let mut html = templates::HEADER.to_owned();
        let markdown = fs::read_to_string(&file.0)?;
        let parser = pulldown_cmark::Parser::new_ext(&markdown, pulldown_cmark::Options::all());
        let modified: DateTime<Local> = file.1.into();

        let mut body = String::new();
        pulldown_cmark::html::push_html(&mut body, parser);

        html.push_str(templates::body(&body, &modified.to_string()).as_str());
        html.push_str(templates::FOOTER);

        let html_file = file
            .0
            .replace(content_dir, output_dir)
            .replace(".md", ".html");

        let folder = Path::new(&html_file).parent().unwrap();
        let _ = fs::create_dir_all(folder);
        fs::write(&html_file, html)?;

        html_files.push((html_file, modified.to_string()));
    }

    write_index(html_files, output_dir)?;
    Ok(())
}

fn write_index(files: Vec<(String, String)>, output_dir: &str) -> SSRGResult<()> {
    let mut html = templates::HEADER.to_owned();
    let body = files
        .into_iter()
        .map(|(file, modified)| {
            let file = file.trim_start_matches(output_dir);
            let title = file.trim_start_matches('/').trim_end_matches(".html");

            format!(
                r#"<small>{}</small> <a href="{}">{}</a>"#,
                modified, file, title
            )
        })
        .collect::<Vec<String>>()
        .join("<br />\n");

    html.push_str(templates::index(&body).as_str());
    html.push_str(templates::FOOTER);

    let index_path = Path::new(&output_dir).join("index.html");

    fs::write(index_path, html)?;
    Ok(())
}
