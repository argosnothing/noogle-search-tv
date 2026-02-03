use crate::data::NoogleResponse;
use anyhow::{Result, anyhow};
use std::process::Command;
use std::env;

pub fn execute(response: &NoogleResponse, name: &str) -> Result<()> {
    let doc = response
        .data
        .iter()
        .find(|d| d.matches_name(name))
        .ok_or_else(|| anyhow!("Function '{}' not found", name))?;

    let position = doc.meta.lambda_position.as_ref()
        .or(doc.meta.attr_position.as_ref())
        .ok_or_else(|| anyhow!("No source position available for '{}'", name))?;

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    
    let location = format!("{}:{}:{}", position.file, position.line, position.column);
    
    Command::new(editor)
        .arg(location)
        .status()?;

    Ok(())
}
