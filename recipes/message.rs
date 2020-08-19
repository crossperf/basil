pub const ID: u32 = 0x93a8_4241;

#[req]
pub struct Load {
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
    #[throws]
    fn handle_request(self, connection: &server::Connection, identity: &UserIdentity) -> Vec<Packed> {
        let recipes = database::Recipe::from_category(connection, &self.category)?
            .into_iter()
            .map(|x| x.into_message(connection))
            .collect::<Result<Vec<Recipe>, Error>>()?;
        vec![pack_set_recipes(recipes)]
    }
}

impl Recipe {
    pub fn from_placeholder() -> Self {
        Self {
            slug: "".to_owned(),
            title: "".to_owned(),
            image: Some(Image::placeholder().with_ratio(50)),
        }
    }

    pub fn from_message<T: Into<String>>(message: T) -> Self {
        Self {
            title: message.into(),
            ..Default::default()
        }
    }

    fn label(&self) -> Option<String> {
        if self.slug.is_empty() {
            None
        } else {
            Some(self.title.clone())
        }
    }

    fn url(&self) -> Option<String> {
        if self.slug.is_empty() && self.title.is_empty() {
            Some("#".to_string())
        } else if self.slug.is_empty() {
            None
        } else {
            Some(format!("/recipe/{}", &self.slug))
        }
    }

    fn message(&self) -> Option<String> {
        if self.slug.is_empty() {
            Some(self.title.clone())
        } else {
            None
        }
    }
}
