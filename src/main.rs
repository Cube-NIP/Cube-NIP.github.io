// SPDX-License-Identifier: Apache-2.0 OR MIT

//! crate 根与主控制流模块。

use core::fmt::Write as _;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let out = Path::new("public");
    fs::create_dir_all(out.join("blog/hello-world"))?;

    let template = fs::read_to_string("templates/base.html")?;
    let markdown = fs::read_to_string("content/blog/hello-world.md")?;
    let body = render_markdown(&markdown);

    let post = template
        .replace("{{title}}", "Hello, world")
        .replace("{{content}}", &body);
    fs::write(out.join("blog/hello-world/index.html"), &post)?;

    let index_body = "<p><a href=\"/blog/hello-world/\">Hello, world</a></p>";
    let index = template
        .replace("{{title}}", "Home")
        .replace("{{content}}", index_body);
    fs::write(out.join("index.html"), index)?;

    fs::copy("static/style.css", out.join("style.css"))?;

    println!("built: index.html, blog/hello-world/index.html, style.css");
    Ok(())
}

// NOTE: 从使用 Markdown 语法的文本转换到使用 HTML 语法的文本
#[inline]
fn render_markdown(markdown: &str) -> String {
    let mut html = String::new();
    for line in markdown.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        if let Some(title) = line.strip_prefix("# ") {
            let _ = writeln!(html, "<h1>{title}</h1>");
        } else if let Some(title) = line.strip_prefix("## ") {
            let _ = writeln!(html, "<h2>{title}</h2>");
        } else if let Some(title) = line.strip_prefix("### ") {
            let _ = writeln!(html, "<h3>{title}</h3>");
        } else if let Some(title) = line.strip_prefix("#### ") {
            let _ = writeln!(html, "<h4>{title}</h4>");
        } else if let Some(title) = line.strip_prefix("##### ") {
            let _ = writeln!(html, "<h5>{title}</h5>");
        } else if let Some(title) = line.strip_prefix("###### ") {
            let _ = writeln!(html, "<h6>{title}</h6>");
        } else {
            let _ = writeln!(html, "<p>{line}</p>");
        }
    }
    html
}
