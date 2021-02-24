#![allow(dead_code)]

use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Wowhead{
    pub item: Item,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub level: u64,
    pub quality: Quality,
    #[serde(rename = "class")]
    pub item_class: String, // TODO: still broken with $value, bug in quick xml?
    // pub item_class: ItemClass,
    pub subclass: String,
    // pub subclass: ItemSubClass,
    pub icon: Icon,
    #[serde(rename = "inventorySlot")]
    pub inventory_slot: InventorySlot,
    #[serde(rename = "htmlTooltip")]
    pub html_tooltip: String,
    pub link: String,
}
#[derive(Debug, Deserialize)]
pub struct Quality {
    pub id: u64,
    #[serde(rename = "$value")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemClass {
    pub id: u64,
    #[serde(rename = "$value")]
    pub name: String,
}
#[derive(Debug, Deserialize)]
pub struct ItemSubClass {
    pub id: u64,
    #[serde(rename = "$value")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Icon {
    #[serde(rename = "displayId")]
    pub display_id: u64,
    #[serde(rename = "$value")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct InventorySlot {
    pub id: u64,
    #[serde(rename = "$value")]
    pub name: Option<String>,
}