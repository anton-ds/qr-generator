use qrcode::QrCode;
use qrcode::render::svg;
use image::{Luma, ImageOutputFormat};
use std::env;
use std::fs;
use std::io::Cursor;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <text> <size> <format (svg/png/webp)>", args[0]);
        std::process::exit(1);
    }

    // parse text (or url), size, format (extension)
    let data = &args[1];
    let size: u32 = args[2].parse().expect("The size should be a number");
    let format = &args[3].to_lowercase();

    // create QR
    let code = QrCode::new(data).expect("Failed to create a QR code");

    match format.as_str() {
        "png" => {
            let image = code.render::<Luma<u8>>()
                .max_dimensions(size, size)
                .build();
            image.save("qrcode.png").expect("Couldn't keep PNG");
            println!("QR code saved as qrcode.png");
        }
        "webp" => {
            let image = code.render::<Luma<u8>>()
                .max_dimensions(size, size)
                .build();
            let mut buffer = Cursor::new(Vec::new());
            image.write_to(&mut buffer, ImageOutputFormat::WebP).expect("Couldn't keep WebP");
            fs::write("qrcode.webp", buffer.into_inner()).expect("Couldn't keep WebP");
            println!("QR code saved as qrcode.webp");
        }
        "svg" => {
            let svg_code = code.render::<svg::Color>()
                .min_dimensions(size, size)
                .build();
            fs::write("qrcode.svg", svg_code).expect("Couldn't keep SVG");
            println!("QR code saved as qrcode.svg");
        }
        _ => {
            eprintln!("Unsupported format: {}. Use svg, png or webp.", format);
            std::process::exit(1);
        }
    }
}
