use rocket::http::ContentType;
use rocket::*;

#[get("/<file>/render")]
fn render(file: &str) -> Result<String, String> {
  let mdcode = std::fs::read_to_string(file).map_err(|e| format!("{}: {}", file, e))?;

  Ok(markdown::to_html(&mdcode))
}

#[get("/<file>")]
fn page(file: &str) -> (ContentType, String) {
  const JSCODE: &str = include_str!("index.js");
  const CSS: &str = include_str!("pico.classless.min.css");
  (
    ContentType::HTML,
    format!(include_str!("index.html"), CSS, file, JSCODE),
  )
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![render, page])
}
