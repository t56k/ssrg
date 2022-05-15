# Individual file writing

A glaring issue in the [previous](https://github.com/t56k/ssrg/commit/b31bc2837e1f4c0ee61ceb7433730af1242d2799) ssrg commit was the fact that the entire site was recompiled on each change. This is even mentioned in Kerkour's original [blog post](https://kerkour.com/rust-static-site-generator) where it is suggested that it should be looked at in future versions. So, let's get to it.

In the `main()` fn, we can easily see how the site is rebuilt on every [hotwatch](https://github.com/francesca64/hotwatch) event:

```
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
```

We need to leverage information stored in `Event` so that we can rebuild whatever file was saved and not the entire site. Thankfully, `hotwatch`'s only example indicates how to destructure the `Event`.

```
if let Event::Write(path) = event {
    println!("{} has changed.", path);
}
```

Accordingly, we can change our code to suit:

```
watch
    .watch(CONTENT, |event: Event| {
        if let Event::Write(path) = event {
            println!("rebuilding {:?}", path);
            // new_fn_to_build(`path`)
        }
    })
    .expect("failed to watch content");
```

That's one problem solved.

Now we need to change the build fns to be able to rebuild one HTML file based on the Markdown's path and not based on the whole `CONTENT` dir. In practical terms, that means splitting the `rebuilt_site()` fn in half--right between the file collection and the block that does the HTML file building. Meaning this rather large fn:

```
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
                    .expect("failed to get modified"),
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

        html.push_str(templates::body(&body, &modified.to_rfc2822()).as_str());
        html.push_str(templates::FOOTER);

        let html_file = file
            .0
            .replace(content_dir, output_dir)
            .replace(".md", ".html");

        let folder = Path::new(&html_file).parent().unwrap();
        let _ = fs::create_dir_all(folder);
        fs::write(&html_file, html)?;

        html_files.push((html_file, modified.to_rfc2822()));
    }

    write_index(html_files, output_dir)?;
    Ok(())
}
```

Can be better expressed as three (well four really) more succinct ones:

```
fn build_site() -> SSRGResult<()> {
    let _ = fs::remove_dir_all(PUBLIC);
    let files = markdown_files()?;
    for file in files {
        write_file(file)?;
    }

    write_index()?;
    Ok(())
}

fn write_file(file: File) -> SSRGResult<()> {
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

fn markdown_files() -> SSRGResult<Vec<File>> {
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
```

Second problem solved.

Building the index required some additional changes. In the original function, it used the same collection of Markdown files from the HTML build which is all well and good for that context, but we can't enjoy that luxury when building individual HTML files. This is because we need to preserve the chronology of the Markdown files and not their respective HTML products--there's little point having a blog where every post is dated by the last build.

Lazily, the simplest way forward is to just use the `markdown_files()` fn and replace the file extensions.

```
fn write_index() -> SSRGResult<()> {
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
```

We're running that `markdown_files()` fn twice now which feels wasteful. Storing the files in a `Vec` to be updated on writes might be a little less costly. Still, could be worse, will be better.
