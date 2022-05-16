pub const HEADER: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" href="css/pico.min.css">
    <title>t56k devlog</title>
    <script async src="https://www.googletagmanager.com/gtag/js?id=G-MVS9VFCDJV"></script>
    <script>
        window.dataLayer = window.dataLayer || [];
        function gtag(){dataLayer.push(arguments);}
        gtag('js', new Date());

        gtag('config', 'G-MVS9VFCDJV');
    </script>
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
        NAV, modified, body
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
        NAV, body
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
    <mark>Development notes from whenever</mark>
"#;

pub const FOOTER: &str = r#"
    <footer class="container">
      <small>Built with <a href="https://github.com/t56k/ssrg" target="_blank">ssrg</a> in 2022 by <a href="https://twitter.com/t56k_" target="_blank">t56k</a></small>
    </footer>
</html>"#;
