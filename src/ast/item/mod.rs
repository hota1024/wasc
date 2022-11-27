pub mod ItemFn;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    ItemFn(ItemFn::ItemFn),
}
