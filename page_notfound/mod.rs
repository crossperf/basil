use crate::prelude::*;

#[derive(Template, Serialize, Deserialize, Module, Debug)]
#[template(name = "page_notfound")]
#[module(wrapper)]
pub struct PageNotFound {}

impl ModuleConstructor for PageNotFound {
    fn new(_id: &mut usize) -> Self {
        Self {}
    }
}
