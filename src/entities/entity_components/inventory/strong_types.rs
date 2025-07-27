use crate::item_utils::item::item::Item;

pub enum AddItem {
    Item(Item),
}
impl AddItem {
    pub fn inner(self) -> Item {
        match self {
            AddItem::Item(item) => item,
        }
    }
}
#[allow(dead_code)]
pub enum RemoveItem {
    Item(Item),
}
#[allow(dead_code)]
impl RemoveItem {
    pub fn inner(self) -> Item {
        match self {
            RemoveItem::Item(item) => item,
        }
    }
}
