use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use native_db::*;
use native_model::{native_model, Model};
use ratatui::text::Text;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Stack {
    #[primary_key]
    pub id: u32,
    pub name: String,
    pub count: u8,
}

impl Clone for Stack {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            count: self.count,
        }
    }
}

impl Stack {
    pub fn save(model: &Stack) -> Result<()> {
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn get_all() -> Vec<Stack> {
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

    pub fn get_by_id(id: u32) -> Option<Result<Stack>> {
        return Some(Ok(Stack {
            id: 0,
            name: "My Stack".to_string(),
            count: 5,
        }));
    }

    pub fn delete(&self) -> Result<()> {
        Ok(())
    }
}
