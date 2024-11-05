// Import necessary libraries
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use marketh_rs::Executor;
use rusttype::{Font, Scale};
use std::path::Path;

// Load the font file
fn load_font() -> Font<'static> {
    // Load a font (.ttf file) from a file. You need to provide a path to a .ttf file
    // For example, use a font like `DejaVuSans.ttf` available on your system.
    let font_data = include_bytes!("../RobotoMono-VariableFont_wght.ttf");
    Font::try_from_bytes(font_data as &[u8]).expect("Error loading font")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    marketh_rs::start_server("127.0.0.1", 4008).await;

    // println!("Hello");
    //
    // let executor = Executor::new();
    //
    // let s = executor
    //     .query_data("evm/1/balance/0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045")
    //     .await?;
    //
    // println!("Balance {s:?}");
    //
    // let s = executor
    //     .query_data("evm/1/erc20_balance/0xF2ec4a773ef90c58d98ea734c0eBDB538519b988/0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045")
    //     .await?;
    //
    // println!("Balance {s:?}");
    //
    Ok(())
}

// Main function to create the image
fn make_image() {
    // Set the dimensions for the image (width x height)
    let width = 100;
    let height = 20;

    // Create a new blank image with a white background
    let mut img = RgbaImage::from_pixel(width, height, Rgba([255, 255, 255, 255]));

    // Draw a colored rectangle in the background
    let background_color = Rgba([200, 200, 250, 255]); // Light blue color
    draw_filled_rect_mut(
        &mut img,
        imageproc::rect::Rect::at(0, 0).of_size(width, height),
        background_color,
    );

    // Load the PNG logo
    let logo_path = Path::new("logo.png"); // Put your logo in the project directory
    let logo = image::open(logo_path).expect("Failed to load image");

    // Resize the logo and overlay it on the background
    let resized_logo = logo.resize(16, 16, image::imageops::FilterType::Lanczos3);
    image::imageops::overlay(&mut img, &resized_logo, 2, 2); // Position at (20, 20)

    // Draw a filled circle on the image
    // let circle_color = Rgba([255, 0, 0, 255]); // Red color
    // draw_filled_circle_mut(&mut img, (400, 300), 50, circle_color); // Position at (400, 300) with radius 50

    // Render text onto the image
    let font = load_font();
    let scale = Scale::uniform(14.0); // Set font size
    let text_color = Rgba([0, 0, 0, 255]); // Black color
    draw_text_mut(&mut img, text_color, 20, 3, scale, &font, "Hello, Rust!");

    // Save the image as PNG
    img.save("output.png").expect("Failed to save image");

    println!("Image created successfully!");
}
