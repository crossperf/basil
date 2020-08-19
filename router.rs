use crate::*;

type Page = Vec<Action>;

pub fn title() -> &'static str {
    "Basil"
}

fn recipes<T: Into<String>>(category: T) -> Page {
    let category = category.into();
    vec![
        Action::SelectPage(site::Show::Recipes),
        Action::Recipes(recipes::Action::Category(category)),
    ]
}

#[get("/")]
fn entree() -> Page {
    recipes("entrée")
}

#[get("/appetizers")]
fn appetizer() -> Page {
    recipes("appetizer")
}

#[get("/desserts")]
fn dessert() -> Page {
    recipes("dessert")
}

#[get("/cocktails")]
fn cocktail() -> Page {
    recipes("cocktail")
}

#[get(_)]
fn not_found() -> Page {
    vec![Action::SelectPage(site::Show::Notfound)]
}

#[derive(num_enum::TryFromPrimitive, Debug, Clone, Copy, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum Role {
    Admin = 0xff,
    Public = 0x00,
}
