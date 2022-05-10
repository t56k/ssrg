# Ripping off the static site generator

I totally stole the generator for this site from Sylvain Kerkour's [blog post](https://kerkour.com/rust-static-site-generator) from last September. While I have customised it a fair bit by changing out the HTTP server to something I'm familiar with, by adding CSS via Pico, and by adding metadata to the Markdown file `Vec`s so that posts have timestamps, the logic of the file conversion remains.

Looking at first-pass issues, the shoehorning of the metadata into a tuple feels pretty clunky:

```
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
    let modified: DateTime<Local> = file.1.modified()?.into();

    let mut body = String::new();
    pulldown_cmark::html::push_html(&mut body, parser);

    html.push_str(templates::body(&body, &modified.to_string()).as_str());

    // ...

    html_files.push((html_file, modified.to_string()));
}
```

I'm not quite sure what a better approach would be at first glance, but it can safely be ignored for the time being.
