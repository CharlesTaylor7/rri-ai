use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
pub struct GameTemplate;

pub trait MyTemplate {
    fn to_html(&self) -> axum::response::Result<Html<String>>;
}

impl<T: Template> MyTemplate for T {
    fn to_html(&self) -> axum::response::Result<Html<String>> {
        match self.render() {
            Ok(html) => Ok(Html(html)),
            Err(err) => Err(format!("askama: {}", err).into()),
        }
    }
}

pub const DAISY_THEMES: [&'static str; 30] = [
    "light",
    "dark",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
    // "synthwave",
    "retro",
    // "cyberpunk",
    "valentine",
    "halloween",
    "garden",
    "forest",
    "aqua",
    "lofi",
    "pastel",
    "fantasy",
    "wireframe",
    "black",
    "luxury",
    "dracula",
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "night",
    "coffee",
    "winter",
    "dim",
    "nord",
    "sunset",
];
