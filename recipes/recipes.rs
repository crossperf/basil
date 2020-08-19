fn ucfirst(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub struct Recipes {
    pub category: String,
    pub first_recipe: gui::Recipe,
    pub first_recipes: Vec<gui::Recipe>,
    pub second_recipes: Vec<gui::Recipe>,
    pub more_recipes: Vec<gui::Recipe>,
}

impl Recipes {
    #[throws]
    fn dispatch(&mut self, action: Action, gui: &mut Gui) {
        if let Action::Category(category) = action {
            gui.title(&format!("{}s", ucfirst(&category)));
            self.category = category.clone();
            self.use_placeholders(gui)?;
            request!(self, gui, message::Load, category);
        }
    }

    #[throws]
    fn use_placeholders(&mut self, gui: &mut Gui) {
        self.set_recipes(vec![message::Recipe::from_placeholder(); 6], gui)?;
    }

    #[res]
    fn set_recipes(&mut self, mut recipes: Vec<message::Recipe>, gui: &mut Gui) {
        self.more_recipes = if recipes.len() >= 6 {
            recipes.insert(
                6,
                message::Recipe::from_message("Gluten-free recipes, curated for your everyday life."),
            );
            recipes.split_off(4).into_iter().map(|x| x.into_gui(gui)).collect()
        } else {
            vec![]
        };
        self.second_recipes = if recipes.len() >= 2 {
            recipes.insert(
                2,
                match self.category.as_str() {
                    "appetizer" => message::Recipe::from_message(
                        "Delicous, light appetizers which will make you smile.",
                    ),
                    "entrée" => message::Recipe::from_message(
                        "Hearty, organic entrées that taste just a little better than homemade.",
                    ),
                    "dessert" => message::Recipe::from_message(
                        "Yummy desserts for every occasion.",
                    ),
                    "cocktail" => message::Recipe::from_message(
                        "Bright cocktails.",
                    ),
                    _ => message::Recipe::from_message("Tastes great."),
                }
            );
            recipes.split_off(3).into_iter().map(|x| x.into_gui(gui)).collect()
        } else {
            vec![]
        };
        self.first_recipe = if recipes.len() >= 1 {
            recipes.remove(0).into_gui(gui)
        } else {
            gui::Recipe::default()
        };
        self.first_recipe.image.as_mut().unwrap().ratio = 0;
        self.first_recipes = recipes.into_iter().map(|x| x.into_gui(gui)).collect();
    }
}

pub enum Action {
    Category(String),
}
