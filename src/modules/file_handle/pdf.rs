use pdf_extract::{OutputError, extract_text};

pub fn read_pdf(path: &str) -> Result<String, OutputError> {
    let file = extract_text(path)?;
    
    Ok(file)
}