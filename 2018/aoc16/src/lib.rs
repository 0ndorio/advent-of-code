pub mod instruction;
pub mod operation;
pub mod state;

use std::error::Error;

pub type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;
