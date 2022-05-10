pub const HEADER: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css">
    <title>t56k devlog</title>
  </head>
"#;

pub fn render_body(body: &str, modified: &str) -> String {
    format!(
        r#"<body>
            <header class="container">
                <nav>
                    <ul>
                        <li><strong><a href="/">t56k devlog</a></strong></li>
                    </ul>
                    <ul>
                        <li><a href="https://github.com/t56k/">github</a></li>
                        <li><a href="https://twitter.com/t56k_/">twitter</a></li>
                    </ul>
                </nav>
            </header>
            <main class="container">
                <h6>{}</h6>
                {}
            </main>
        </body>"#,
        modified,
        body
    )
}

pub fn render_index(body: &str) -> String {
    format!(
        r#"<body>
            <header class="container">
                <nav>
                    <ul>
                        <li><strong><a href="/">t56k devlog</a></strong></li>
                    </ul>
                    <ul>
                        <li><a href="https://github.com/t56k/">github</a></li>
                        <li><a href="https://twitter.com/t56k_/">twitter</a></li>
                    </ul>
                </nav>
            </header>
            <main class="container">
                {}
            </main>
        </body>"#,
        body
    )
}

pub const FOOTER: &str = r#"</html>"#;
