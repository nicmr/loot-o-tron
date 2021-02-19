#![allow(dead_code)]

use serde::{Deserialize};


#[derive(Deserialize)]
pub struct Wowhead{
    pub item: Item,
}

#[derive(Deserialize)]
pub struct Item {
    pub name: String,
    pub level: u64,
    pub quality: Quality,
    pub class: ItemClass,
    pub subclass: ItemSubClass,
    pub icon: Icon,
    #[serde(rename = "inventorySlot")]
    pub inventory_slot: InventorySlot,
    #[serde(rename = "htmlTooltip")]
    pub html_tooltip: String,
    pub link: String,
}
#[derive(Deserialize)]
pub struct Quality {
    id: u64,
    #[serde(rename = "$value")]
    name: String,
}

#[derive(Deserialize)]
pub struct ItemClass {
    id: u64,
    #[serde(rename = "$value")]
    name: String,
}
#[derive(Deserialize)]
pub struct ItemSubClass {
    id: u64,
    #[serde(rename = "$value")]
    name: String,
}

#[derive(Deserialize)]
pub struct Icon {
    #[serde(rename = "displayId")]
    display_id: u64,
    #[serde(rename = "$value")]
    name: String,
}

#[derive(Deserialize)]
pub struct InventorySlot {
    id: u64,
    #[serde(rename = "$value")]
    name: String,
}