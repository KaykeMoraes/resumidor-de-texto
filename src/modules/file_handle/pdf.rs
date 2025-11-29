use anyhow::{Error, Ok, anyhow};
use pdf_extract::{extract_text};

pub fn read_pdf(path: &str) -> Result<String, Error> {
    let file = extract_text(path);
    if file.is_err() {
        Err(anyhow!("Erro ao ler arquivo pdf."))
    } else {
        Ok(file.ok().unwrap())
    }
}