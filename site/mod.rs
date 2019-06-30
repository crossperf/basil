use crate::prelude::*;

oneof! {
    SelectPage {
        PageNotFound,
        Recipes,
    }
}

#[derive(Template, Serialize, Deserialize, Module, Debug)]
#[module(wrapper)]
pub struct Site {
    pub page: SelectPage,
}

impl ModuleConstructor for Site {
    fn new(id: &mut usize) -> Self {
        Self {
            page: SelectPage::new(id),
        }
    }
}
