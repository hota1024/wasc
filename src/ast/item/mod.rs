pub mod item_fn;
pub mod item_import;

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    ItemFn(item_fn::ItemFn),
    ItemImport(item_import::ItemImport),
}
