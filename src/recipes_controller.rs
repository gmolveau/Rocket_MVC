use rocket_contrib::Template;
// use models;

#[get("/")]
pub fn index() -> Template {
    let map = ();
    Template::render("index", &map)
}

#[get("/<name>")]
pub fn show(name: String) -> Template {
    // let recipe = models::Recipe{name: name};
    let recipe = ();
    Template::render("recipe", recipe)
}