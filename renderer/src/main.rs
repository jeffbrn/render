use image::{RgbImage, Rgb};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

fn main() {
    // Create a blank image with white background
    let mut img = RgbImage::new(400, 300);
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]); // White color
    }

    // Define a rectangle
    let rect = Rect::at(50, 50).of_size(200, 100);

    // Draw a filled rectangle with red color
    draw_filled_rect_mut(&mut img, rect, Rgb([255, 0, 0]));

    // Save the image as a PNG file
    img.save("output.png").expect("Failed to save image");

    println!("Image saved as output.png");
}
