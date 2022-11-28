pub mod item_fn;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    ItemFn(item_fn::ItemFn),
}
