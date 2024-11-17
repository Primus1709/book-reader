use actix_web::{get, Responder};
use epub::doc::EpubDoc;

/// Handler to parse the uploaded EPUB file and extract metadata or basic information.
#[get("/parse")]
pub async fn parse() -> impl Responder {
    let path = "./uploads/uploaded.epub"; // Path to the uploaded EPUB file

    match EpubDoc::new(path) {
        Ok(doc) => {
            // Attempt to extract the book title as a fallback
            let title = doc.mdata("title").unwrap_or("Unknown Title".to_string());
            format!("Book Title: {}", title)
        }
        Err(_) => "Failed to parse EPUB".to_string(), // Return error message if parsing fails
    }
}
