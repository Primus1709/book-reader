use actix_multipart::Multipart;
use actix_web::{post, Responder, HttpResponse};
use futures_util::StreamExt as _;
use std::io::Write;
use std::fs::File;
use std::path::Path;

#[post("/upload")]
pub async fn upload(mut payload: Multipart) -> impl Responder {
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();

        // Get the filename from the multipart data
        let filename = match field.content_disposition().get_filename() {
            Some(name) => name.to_string(),
            None => return HttpResponse::BadRequest().body("Filename missing in the uploaded file."),
        };

        // Define the path where the file will be saved
        let filepath = format!("./uploads/{}", filename);
        let path = Path::new(&filepath);

        // Create the directory if it does not exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|_| {
                panic!("Failed to create upload directory!");
            });
        }

        // Open the file to write data
        let mut f = match File::create(&filepath) {
            Ok(file) => file,
            Err(_) => return HttpResponse::InternalServerError().body("Failed to create file."),
        };

        // Write file data in chunks
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            if let Err(e) = f.write_all(&data) {
                return HttpResponse::InternalServerError().body(format!("Failed to write file: {}", e));
            }
        }
    }

    HttpResponse::Ok().body("File uploaded successfully")
}
