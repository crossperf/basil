oneof! {
    SelectPage {
        Notfound,
        Recipes,
    }
}

pub struct Site {
    pub select_page: SelectPage,
}
