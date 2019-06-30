use super::Recipe;
use crate::prelude::*;

pub const ID: u32 = 0x93a8_4241;

#[derive(TryFromPrimitive, Payload)]
#[repr(u8)]
#[payload(Recipes)]
pub enum PayloadType {
    #[payload(request)]
    Load,
    #[payload(response)]
    Data,
}

#[derive(Message, Pack, Default, Clone)]
#[pack(ID, Load)]
pub struct Load {
    #[prost(string, tag = "1")]
    pub category: String,
}

impl Load {
    pub fn new<T: Into<String>>(category: T) -> Self {
        Self {
            category: category.into(),
        }
    }
}

#[cfg(feature = "server")]
impl Request for Load {
    fn handle_request(&self, connection: &server::Connection) -> Result<Vec<Packed>, Error> {
        let data = database::recipes(connection, &self.category)?;
        Ok(vec![data.pack()])
    }
}

#[derive(Message, Pack, Default)]
#[pack(ID, Data)]
pub struct Data {
    #[prost(message, repeated, tag = "1")]
    pub recipes: Vec<Recipe>,
}

impl Data {
    fn handle_response(self, state: &mut Recipes) {
        state.set_recipes(self.recipes);
    }
}

#[cfg(feature = "server")]
mod database {
    use super::{Data, Recipe};
    use crate::interactive::Image;
    use diesel::QueryResult;
    use server::Connection;

    #[derive(Queryable)]
    struct RawRecipe {
        slug: String,
        title: String,
        image: String,
    }

    impl RawRecipe {
        fn into_item(self, connection: &Connection) -> QueryResult<Recipe> {
            Ok(Recipe {
                label: Some(self.title),
                url: Some(format!("/recipe/{}", &self.slug)),
                message: None,
                image: Some(
                    Image::from_database(connection, &self.image)?
                        .unwrap_or_else(|| Image::placeholder()).with_ratio(50)
                ),
            })
        }
    }

    pub fn recipes(connection: &Connection, filter_category: &str) -> QueryResult<Data> {
        use crate::schema::*;
        use diesel::prelude::*;
        let recipes = recipe::table
            .filter(recipe::category.eq(filter_category))
            .select((recipe::slug, recipe::title, recipe::image))
            .load::<RawRecipe>(connection)?;
        let recipes = recipes
            .into_iter()
            .map(|recipe| recipe.into_item(connection))
            .collect::<QueryResult<Vec<Recipe>>>()?;
        Ok(Data { recipes })
    }
}
