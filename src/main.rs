use std::net::IpAddr;
use std::str::FromStr;
use std::time::SystemTime;
use std::{fs, path::Path, thread, time::Duration};

use chrono::{DateTime, Local};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use hotwatch::{Hotwatch, Event};
use warp::Filter;

mod templates;

const CONTENT: &str = "content";
const PUBLIC: &str = "public";
const IP: &str = "127.0.0.1";

type SSRGResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> SSRGResult<()> {
    println!("building files");
    rebuild_site(CONTENT, PUBLIC)?;

    tokio::task::spawn_blocking(move || {
        let mut watch = Hotwatch::new().expect("hotwatch failed");

        watch
            .watch(CONTENT, |_event: Event| {
                println!("rebuilding files");
                rebuild_site(CONTENT, PUBLIC).expect("rebuilding");
            })
            .expect("failed to watch content");

        loop {
            thread::sleep(Duration::from_secs(1));
        }
    });

    let addr = IpAddr::from_str(&*IP)?;
    let index = warp::get()
        .and(warp::fs::dir(PUBLIC));

    warp::serve(index).run((addr, 4000)).await;
    Ok(())
}

fn rebuild_site(content_dir: &str, output_dir: &str) -> SSRGResult<()> {
    let _ = fs::remove_dir_all(output_dir);

    let markdown_files: Vec<(String, fs::Metadata)> = walkdir::WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().display().to_string().ends_with(".md"))
        .map(|e| (e.path().display().to_string(), e.metadata().unwrap()))
        .collect();

    let mut html_files = Vec::with_capacity(markdown_files.len());

    for file in &markdown_files {
        let mut html = templates::HEADER.to_owned();
        let markdown = fs::read_to_string(&file.0)?;
        let parser = pulldown_cmark::Parser::new_ext(&markdown, pulldown_cmark::Options::all());
        let modified = file.1.modified().unwrap();
        let timestamp: DateTime<Local> = modified.into();
        let ht = HumanTime::from(timestamp);
        let time_str = ht.to_text_en(Accuracy::Rough, Tense::Past);

        let mut body = String::new();
        pulldown_cmark::html::push_html(&mut body, parser);

        html.push_str(templates::render_body(&body, &time_str).as_str());
        html.push_str(templates::FOOTER);

        let html_file = file.0
            .replace(content_dir, output_dir)
            .replace(".md", ".html");

        let folder = Path::new(&html_file).parent().unwrap();
        let _ = fs::create_dir_all(folder);
        fs::write(&html_file, html)?;

        html_files.push((html_file, modified));
    }

    write_index(html_files, output_dir)?;
    Ok(())
}

fn write_index(files: Vec<(String, SystemTime)>, output_dir: &str) -> SSRGResult<()> {
    let mut html = templates::HEADER.to_owned();
    let body = files
        .into_iter()
        .map(|(file, modified)| {
            let file = file.trim_start_matches(output_dir);
            let title = file.trim_start_matches("/").trim_end_matches(".html");
            let timestamp: DateTime<Local> = modified.into();
            let ht = HumanTime::from(timestamp);
            let time_str = ht.to_text_en(Accuracy::Rough, Tense::Past);

            format!(r#"{} <a href="{}">{}</a>"#, time_str, file, title)
        })
        .collect::<Vec<String>>()
        .join("<br />\n");

    html.push_str(templates::render_index(&body).as_str());
    html.push_str(templates::FOOTER);

    let index_path = Path::new(&output_dir).join("index.html");

    fs::write(index_path, html)?;
    Ok(())
}
