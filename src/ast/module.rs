use super::item::Item;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub items: Vec<Item>,
}
