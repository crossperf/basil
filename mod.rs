modules! {
    base::Base,
    page_notfound::PageNotFound,
    site::Site,
    recipes::Recipes,
}
server! {
    recipes,
}

#[derive(Debug)]
pub enum Action {
    Recipes(recipes::Action),
    SelectPage(site::Show),
    Event(client::Event),
    Response(u64, u8, Vec<u8>, bool),
    Stage(std::collections::HashSet<usize>),
    Go(String),
    Interactive,
    Tick,
}
