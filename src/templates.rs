use tera::Tera;

/// Global template engine, initialized once from the templates directory.
pub fn engine() -> Tera {
    let mut tera = Tera::new();
    tera.autoescape_on(vec!["html"]);
    tera.add_template_files(vec![
        ("templates/base.html", Some("base.html")),
        ("templates/index.html", Some("index.html")),
    ])
    .expect("failed to load templates");
    tera
}
