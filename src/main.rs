use anyhow::Result;

pub mod db;
pub mod parser;

fn main() -> Result<()> {
    db::init();
    return parser::init();
}
