# **eBook Reader**

A simple cross-platform eBook reader implemented in Rust using the Actix Web framework. This project allows users to upload EPUB files, parse their content, and serve a user-friendly interface via a web browser.

---

## **Features**
- Upload EPUB files.
- Parse EPUB metadata and basic content (e.g., chapter titles).
- User-friendly web interface.
- Built with Rust for performance and reliability.

---

## **Prerequisites**
1. Rust (latest stable version). Install it from [Rust's official website](https://www.rust-lang.org/tools/install).
2. `cargo` package manager (installed with Rust).
3. A browser to access the frontend interface.

---

## **File Structure**

```plaintext
ebook_reader/
├── src/
│   ├── handlers/
│   │   ├── mod.rs          # Module definitions
│   │   ├── upload.rs       # File upload handler
│   │   └── parse.rs        # EPUB parsing handler
│   ├── routes/
│   │   └── mod.rs          # Route configuration
│   ├── main.rs             # Application entry point
├── static/
│   ├── index.html          # Frontend interface
├── uploads/
│   └── uploaded.epub       # Directory for uploaded files (auto-created)
├── Cargo.toml              # Rust project manifest
├── README.md               # Project documentation
