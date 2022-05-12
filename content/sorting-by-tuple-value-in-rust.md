# Sorting by tuple value in Rust

In publishing my last post, I realised my `Vec<(String, fs::Metadata)>` wasn't sorted by the second tuple value, meaning the posts in the index don't appear in chronological order. Obviously, for a blog, this is just plain wrong.

So, at first glance, there appear to be a couple of options available to sort them--i.e., either I sort `markdown_files` which'd require implementing `Ord`, `PartialOrd` and `PartialEq` for `fs::Metadata`, or I could sort the `html_files` vector naturally since it's a `Vec<(String, String)>`.

```
fn rebuild_site(content_dir: &str, output_dir: &str) -> SSRGResult<()> {
    let _ = fs::remove_dir_all(output_dir);

    let markdown_files: Vec<(String, fs::Metadata)> = walkdir::WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().display().to_string().ends_with(".md"))
        .map(|e| (e.path().display().to_string(), e.metadata().expect("failed to get metadata")))
        .collect();

    let mut html_files = Vec::with_capacity(markdown_files.len());

    for file in &markdown_files {
        let mut html = templates::HEADER.to_owned();
        let markdown = fs::read_to_string(&file.0)?;
        let parser = pulldown_cmark::Parser::new_ext(&markdown, pulldown_cmark::Options::all());
        let modified: DateTime<Local> = file.1.modified()?.into();

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
```

Sorting `html_files` with something like `html_files.sort_by_key(|k| k.1);` returns an error:

```
html_files.sort_by_key(|k| k.1);
                           ^^^ move occurs because `k.1` has type `std::string::String`, which does not implement the `Copy` trait
```

Implementing `Copy` for `String` seems like overkill here.

A better solution appears to be extracting the `modified()` value when collecting `markdown_files`, then sorting the resulting `SystemTime`.

```
let mut markdown_files: Vec<(String, SystemTime)> = walkdir::WalkDir::new(content_dir)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().display().to_string().ends_with(".md"))
    .map(|e| (e.path().display().to_string(), e.metadata().expect("failed to get metadata").modified().unwrap()))
    .collect();

markdown_files.sort_by_key(|file| file.1);
```

Better, but backwards.

```
markdown_files.sort_by_key(|file| file.1);
markdown_files.reverse();
```

Done. Blogging's better when we start with the newest post.
