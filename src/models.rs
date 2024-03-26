use chrono::{DateTime, Utc};
use native_db::*;
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// #[native_model(id = 1, version = 1)]
// #[native_db]
// pub struct Stack {
//     #[primary_key]
//     id: u32,
//     name: String,
//     count: u8
// }

pub mod stack;




#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[native_model(id = 2, version = 1)]
#[native_db]
pub struct StackEntry {
    #[primary_key]
    id: u32,
    #[secondary_key]
    stack_id: u32,
    entry: String,

}