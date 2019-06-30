pub mod message;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Message, Clone, Default)]
pub struct Recipe {
    #[prost(string, optional, tag = "1")]
    pub url: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub label: Option<String>,
    #[prost(message, optional, tag = "3")]
    pub image: Option<Image>,
    #[prost(string, optional, tag = "4")]
    pub message: Option<String>,
}

impl Recipe {
    pub fn from_message<T: Into<String>>(message: T) -> Self {
        Self {
            url: None,
            label: None,
            image: None,
            message: Some(message.into()),
        }
    }

    pub fn placeholder() -> Self {
        Self {
            url: Some("#".to_owned()),
            label: Some("".to_owned()),
            image: Some(Image::placeholder().with_ratio(50)),
            message: None,
        }
    }
}

#[derive(Template, Serialize, Deserialize, Module, Debug)]
#[module(request)]
#[template(name = "recipes")]
pub struct Recipes {
    pub category: String,
    pub first_recipe: Recipe,
    pub first_recipes: Vec<Recipe>,
    pub second_recipes: Vec<Recipe>,
    pub more_recipes: Vec<Recipe>,
    pub moduleid: usize,
    pub pending_requests: HashSet<u64>,
}

impl ModuleConstructor for Recipes {
    fn new(id: &mut usize) -> Self {
        Self {
            category: String::new(),
            first_recipe: Recipe::default(),
            first_recipes: vec![],
            second_recipes: vec![],
            more_recipes: vec![],
            moduleid: next(id),
            pending_requests: HashSet::new(),
        }
    }
}

impl Recipes {
    fn on_event(&mut self, _event: &client::Event, _actions: &mut Vec<Action>) -> Result<(), Error> {
        Ok(())
    }

    fn dispatch(&mut self, action: Action, gui: &mut Gui) -> Result<(), Error> {
        if let Action::Category(category) = action {
            self.category = category.clone();
            self.use_placeholders();
            gui.request(&message::Load::new(category), &mut self.pending_requests)?;
        }
        Ok(())
    }

    fn use_placeholders(&mut self) {
        self.set_recipes(vec![Recipe::placeholder(); 6]);
    }

    fn set_recipes(&mut self, mut recipes: Vec<Recipe>) {
        self.more_recipes = if recipes.len() >= 6 {
            recipes.insert(
                6,
                Recipe::from_message("Gluten-free recipes, curated for your everyday life."),
            );
            recipes.split_off(4)
        } else {
            vec![]
        };
        self.second_recipes = if recipes.len() >= 2 {
            recipes.insert(
                2,
                match self.category.as_str() {
                    "appetizer" => Recipe::from_message(
                        "Delicous, light appetizers which will make you smile.",
                    ),
                    "entrée" => Recipe::from_message(
                        "Hearty, organic entrées that taste just a little better than homemade.",
                    ),
                    "dessert" => Recipe::from_message(
                        "Yummy desserts for every occasion.",
                    ),
                    "cocktail" => Recipe::from_message(
                        "Bright cocktails.",
                    ),
                    _ => Recipe::from_message("Tastes great."),
                }
            );
            recipes.split_off(3)
        } else {
            vec![]
        };
        self.first_recipe = if recipes.len() >= 1 {
            recipes.remove(0)
        } else {
            Recipe::default()
        };
        self.first_recipe.image.as_mut().unwrap().ratio = 0;
        self.first_recipes = recipes;
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Category(String),
    Request(message::Payload),
    Refresh,
    Nop,
}
