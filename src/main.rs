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

mod helpers;
mod templates;

const CONTENT: &str = "content";
const PUBLIC: &str = "public";

type File = (String, SystemTime);
type ErrRes<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> ErrRes<()> {
    build_site()?;

    tokio::task::spawn_blocking(move || {
        let mut watch = Hotwatch::new().expect("hotwatch failed");

        watch
            .watch(CONTENT, |event: Event| {
                if let Event::Write(path) = event {
                    println!("rebuilding {:?}", path);

                    let now = SystemTime::now();
                    write_file((path.into_os_string().into_string().unwrap(), now))
                        .expect("couldn't write file");

                    write_index().expect("couldn't rewrite index");
                }
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

fn build_site() -> ErrRes<()> {
    let files = markdown_files()?;
    for file in files {
        write_file(file)?;
    }

    write_index()?;
    Ok(())
}

fn write_file(file: File) -> ErrRes<()> {
    let mut html = templates::HEADER.to_owned();
    let markdown = fs::read_to_string(&file.0)?;
    let parser = pulldown_cmark::Parser::new_ext(&markdown, pulldown_cmark::Options::all());
    let modified: DateTime<Local> = file.1.into();

    let mut body = String::new();
    pulldown_cmark::html::push_html(&mut body, parser);

    html.push_str(templates::body(&body, &modified.to_rfc2822()).as_str());
    html.push_str(templates::FOOTER);

    let html_file = file.0.replace(CONTENT, PUBLIC).replace(".md", ".html");
    let folder = Path::new(&html_file).parent().unwrap();
    let _ = fs::create_dir_all(folder);
    fs::write(&html_file, html)?;

    Ok(())
}

fn write_index() -> ErrRes<()> {
    let files = markdown_files()?;

    let mut html = templates::HEADER.to_owned();
    let body = files
        .into_iter()
        .map(|(file, modified)| {
            let file_name = file.trim_start_matches(CONTENT).replace(".md", ".html");
            let clean_file_name = file_name.trim_start_matches('/').trim_end_matches(".html");
            let title = str::replace(clean_file_name, '-', " ");
            let relative_mod: DateTime<Local> = modified.into();

            format!(
                r#"<small>{}</small><br /><a href="{}">{}</a><hr />"#,
                relative_mod.to_rfc2822(),
                file_name,
                helpers::titlize(&title)
            )
        })
        .collect::<Vec<String>>()
        .join("<br />\n");

    html.push_str(templates::index(&body).as_str());
    html.push_str(templates::FOOTER);

    let index_path = Path::new(&PUBLIC).join("index.html");
    fs::write(index_path, html)?;

    Ok(())
}

fn markdown_files() -> ErrRes<Vec<File>> {
    let mut files: Vec<File> = walkdir::WalkDir::new(&CONTENT)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().display().to_string().ends_with(".md"))
        .map(|e| {
            (
                e.path().display().to_string(),
                e.metadata()
                    .expect("failed to get metadata")
                    .modified()
                    .expect("failed to get modified"),
            )
        })
        .collect();

    files.sort_by_key(|file| file.1);
    files.reverse();

    Ok(files)
}
