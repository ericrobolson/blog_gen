use super::ContentIr;
use crate::Item;

#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub contents: Item<ContentIr>,
    pub file_name: String,
    pub keywords: Vec<String>,
    pub summary: String,
    pub title: String,
}
