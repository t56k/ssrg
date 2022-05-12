pub const HEADER: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css">
    <title>t56k devlog</title>
  </head>
"#;

pub fn body(body: &str, modified: &str) -> String {
    format!(
        r#"<body>
            <header class="container">
                {}
            </header>
            <main class="container">
                <small>{}</small>
                {}
            </main>
        </body>"#,
        NAV,
        modified,
        body
    )
}

pub fn index(body: &str) -> String {
    format!(
        r#"<body>
            <header class="container">
                {}
            </header>
            <main class="container">
                {}
            </main>
        </body>"#,
        NAV,
        body
    )
}

pub const NAV: &str = r#"
    <nav>
        <ul>
            <li><strong><a href="/">t56k devlog</a></strong></li>
        </ul>
        <ul>
            <li><a href="https://github.com/t56k/" target="_blank">gh</a></li>
            <li><a href="https://stackoverflow.com/users/1153022/t56k" target="_blank">so</a></li>
            <li><a href="https://twitter.com/t56k_/" target="_blank">tw</a></li>
        </ul>
    </nav>
    <mark>Development notes from whenever I think to update them</mark>
"#;

pub const FOOTER: &str = r#"
    <footer class="container">
      <small>© 2022 <a href="https://twitter.com/t56k_" target="_blank">t56k</a></small>
    </footer>
</html>"#;
