use color_eyre::eyre::Result;
use chrono::{DateTime, Utc};
use native_db::*;
use native_model::{native_model, Model};
use ratatui::text::Text;
use serde::{Deserialize, Serialize};

use crate::db_reader::DbReader;



#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Stack {
    #[primary_key]
    pub id: u32,
    pub name: String,
    pub count: u8
}

impl DbReader<Stack> for Stack {
    fn save(model: &Stack) -> Result<()> {
       
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_all() -> Vec<Stack> {
        let stacks = vec![
            Stack {
                id: 1,
                name: "Stack 1".to_string(),
                count: 1,
            },
            Stack {
                id: 2,
                name: "Stack 2".to_string(),
                count: 5,
            },
            Stack {
                id: 3,
                name: "Stack 3".to_string(),
                count: 3,
            },
            Stack {
                id: 4,
                name: "Stack 4".to_string(),
                count: 8,
            },
        ];

        return stacks;
                 
    }

    fn get_by_id(id: u32) -> Option<Result<Stack>> {
        return Some(Ok(Stack {
            id: 0,
            name: "My Stack".to_string(),
            count: 5
        }));
    }

    fn delete(&self) -> Result<()> {
        Ok(())
    }
}





