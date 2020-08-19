#[derive(Queryable)]
pub struct Recipe {
    slug: String,
    title: String,
    image: String,
}

impl Recipe {
    #[throws]
    pub fn from_category(connection: &Connection, filter_category: &str) -> Vec<Self> {
        use crate::schema::*;
        use diesel::prelude::*;
        let recipes = recipe::table
            .filter(recipe::category.eq(filter_category))
            .select((recipe::slug, recipe::title, recipe::image))
            .load::<Recipe>(connection)?;
        recipes
    }

    #[throws]
    fn image(&self, connection: &Connection) -> Option<Image> {
        Some(
            Image::from_database(connection, &self.image)?
                .unwrap_or_else(|| Image::placeholder()).with_ratio(50)
        )
    }
}
