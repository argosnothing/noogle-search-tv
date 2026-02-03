use crate::data::NoogleResponse;
use crate::format;
use anyhow::Result;

pub fn execute(response: &NoogleResponse, input: &str) -> Result<()> {
    let doc = super::util::find_doc(response, input)?;
    format::print_preview(doc);
    Ok(())
}
