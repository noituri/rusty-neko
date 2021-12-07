use crate::structures::column::Column;

pub struct Table {
    pub columns: Vec<Column>,
    pub name: String,
}
