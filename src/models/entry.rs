use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use native_db::*;
use native_model::{native_model, Model};
use ratatui::text::Text;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[native_model(id = 2, version = 1)]
#[native_db]
pub struct Entry {
    #[primary_key]
    pub id: u32,
    #[secondary_key]
    pub stack_id: u32,
    pub content: String,
}

impl Entry {
    pub fn save(model: &Entry) -> Result<()> {
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn get_all(id: &u32) -> Vec<Entry> {
        let entries = vec![
            Entry {
                id: 1,
                stack_id: id.clone(),
                content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string(),
            },
            Entry {
                id: 2,
                stack_id: id.clone(),
                content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla id imperdiet dolor. Proin a dolor sit amet erat viverra condimentum. Praesent maximus efficitur ante, a cursus dui suscipit id."
                    .to_string(),
            },
            Entry {
                id: 3,
                stack_id: id.clone(),
                content: "Quisque et mattis ex. Phasellus dignissim dignissim congue. Nam nec maximus elit, vitae rutrum leo. Nullam ultrices lobortis leo, in ultricies justo viverra vulputate. Nulla non accumsan lectus. Mauris ac sapien auctor, posuere orci nec, maximus quam. In commodo, diam id tristique pellentesque, mauris justo semper nisl, facilisis venenatis enim ligula nec eros. Suspendisse potenti. Nunc id condimentum magna. Etiam blandit eleifend neque ac posuere. Maecenas magna metus, sodales vitae lorem nec, facilisis consectetur purus. Sed mollis vel enim at imperdiet. Maecenas nulla velit, iaculis sed pretium ultricies, auctor sit amet orci."
                    .to_string(),
            },
            Entry {
                id: 4,
                stack_id: id.clone(),
                content: "Duis aute irure dolor."
                    .to_string(),
            },
        ];

        entries
    }

    pub fn get_by_id(id: u32) -> Option<Result<Entry>> {
        None
    }

    pub fn delete(&self) -> Result<()> {
        Ok(())
    }
}
