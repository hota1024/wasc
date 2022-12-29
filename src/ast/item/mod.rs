pub mod item_fn;
pub mod item_import;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    ItemFn(item_fn::ItemFn),
    ItemImport(item_import::ItemImport),
}
