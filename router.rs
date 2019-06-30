use http_router::Method;
use lazy_static::lazy_static;

use crate::{site, recipes, Action};

type Context = ();
type Page = (u16, String, Vec<Action>);

fn ucfirst(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[allow(clippy::redundant_closure_call)]
pub fn actions_for(path: &str) -> Page {
    type Router = fn(Context, Method, &str) -> Page;
    lazy_static! {
        static ref ROUTE: Router = router!(
            GET / => entree,
            GET /entrees => entree,
            GET /appetizers => appetizer,
            GET /desserts => dessert,
            GET /cocktails => cocktail,
            _ => not_found,
        );
    }
    ROUTE((), Method::GET, path)
}

fn recipes<T: Into<String>>(category: T) -> Page {
    let category = category.into();
    let name = ucfirst(&category);
    let actions = vec![
        Action::SelectPage(site::Show::Recipes),
        Action::Recipes(recipes::Action::Category(category)),
    ];
    (200, format!("Basil - {}s", name), actions)
}

fn entree(_ctx: &Context) -> Page {
    recipes("entrée")
}

fn appetizer(_ctx: &Context) -> Page {
    recipes("appetizer")
}

fn dessert(_ctx: &Context) -> Page {
    recipes("dessert")
}

fn cocktail(_ctx: &Context) -> Page {
    recipes("cocktail")
}

fn not_found(_ctx: &Context) -> Page {
    let actions = vec![Action::SelectPage(site::Show::PageNotFound)];
    (404, "404 Not Found".to_owned(), actions)
}
