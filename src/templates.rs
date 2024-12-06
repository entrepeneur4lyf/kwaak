use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera =
        Tera::new("templates/**/*").expect("Failed to create Tera instance");
}
