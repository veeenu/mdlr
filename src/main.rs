use axum::{extract::Path, response::Html, routing::get, Router, Server};
use pulldown_cmark::{Options, Parser};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/render/*path", get(serve))
        .route("/*path", get(index));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(Path(path): Path<String>) -> Html<String> {
    const JSCODE: &str = include_str!("index.js");
    const CSS: &str = include_str!("pico.classless.min.css");
    Html(format!(include_str!("index.html"), CSS, path, JSCODE))
}

async fn serve(Path(path): Path<String>) -> Result<Html<String>, Html<String>> {
    let path = path.trim_start_matches('/');

    let mdcode = std::fs::read_to_string(path).map_err(|e| Html(format!("{}: {}", path, e)))?;
    let parser = Parser::new_ext(&mdcode, Options::all());
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    Ok(Html(html_output))
}
