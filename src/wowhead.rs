#![allow(dead_code)]

use std::collections::HashMap;

use log::{debug, error};
use quick_xml::Reader;
use serde::{Deserialize};

use crate::error::WowheadError;

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
    pub subclass: String,  // TODO: still broken with $value, bug in quick xml?
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


#[derive(Debug, Deserialize)]
pub struct HtmlTooltip {
    #[serde(rename = "table")]
    tables: Vec<Table>,
}

#[derive(Debug, Deserialize)]
pub struct Table {

}


pub fn parse_html_tooltip (html_tooltip: &str) -> Result<HashMap<String, String>, WowheadError> {

    let mut attributes = HashMap::new();

    // Parse the item stats from the html tooltip
    let mut item_tooltip_reader = Reader::from_str(html_tooltip);
    // This is html, which contains <br> tags, which don't have corresponding end names, so we have to disable the check for this reader.
    item_tooltip_reader.check_end_names(false);

    let mut buf  = Vec::new();
    loop {
        use quick_xml::events::Event;
        match item_tooltip_reader.read_event(&mut buf) {
            // Ok(Event::Start(ref e)) => {
            //     let mut name = String::new();
            //     e.name().read_to_string(&mut name)?;
            //     debug!("attr: {:?}", name);
            // },
            Ok(Event::Comment(c)) => {
                let comment = c.unescape_and_decode(&item_tooltip_reader)?;
                match comment.as_str() {
                    "ilvl" => {
                        match item_tooltip_reader.read_event(&mut buf) {
                            Ok(Event::Text(t)) => {
                                let ilvl = t.unescape_and_decode(&item_tooltip_reader)?;
                                attributes.insert("iLvl".to_owned(), ilvl);
                            },
                            Ok(_) => {
                                error!("Encountered unexpected element after marker comment");
                            },
                            Err(_) => {}
                        }

                    }
                    _ => {
                        debug!("Ignoring comment: {:?}", comment)
                    }
                }
            },
            Ok(Event::Text(t)) => {
                let text = t.unescape_and_decode(&item_tooltip_reader)?;
                if text.len() > 0 {
                    debug!("text: {:?}", text);
                }
                // txt.push(e.unescape_and_decode(&reader).unwrap()),
            }, 
            Ok(Event::Eof) => break,
            Ok (_) => {}, //ignore irrelevant events
            Err(e) => {
                error!("{:?}", e);
            }
        }
        buf.clear();
    }
    return Ok(attributes);
    
}