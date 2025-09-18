use std::path::Path;
use qrcode::QrCode;
use image::Luma;
use chrono::Local;
use std::io::{self, Write};

fn main() {
    let mut link = String::new();

    print!("Enter the link: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut link).expect("Error reading input");

    let link = link.trim();

    let code = QrCode::new(link.as_bytes()).unwrap();

    let image = code.render::<Luma<u8>>().build();

    let now = Local::now();
    let timestamp = now.format("%Y%m%d_%H%M%S").to_string();

    let filename = format!("output/qrcode_{}.png", timestamp);
    let path = Path::new(&filename);

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    image.save(path).unwrap();

    println!("QR code saved at {:?}", path);
}
