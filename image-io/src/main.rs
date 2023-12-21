use image::{GenericImageView, DynamicImage, Rgba};

fn main() {
    // Read an image file
    let img = image
                            ::open("E:\\rust\\srk.jpg")
                            .expect("Failed to open image");

    // Get image dimensions
    let (width, height) = img.dimensions();
    println!("Image dimensions: {} x {}", width, height);

    // Access pixels and modify them (e.g., convert to grayscale)
    let gray_img = img.grayscale();

    // Save the modified image
    gray_img
        .save("E:\\rust\\image_gray.jpg")
        .expect("Failed to save image");

    // // Create a new image and save it
    // let mut new_img = DynamicImage::new_rgba8(width, height);
    // for y in 0..height {
    //     for x in 0..width {
    //         let pixel = img.get_pixel(x, y);
    //         new_img.put_pixel(x, y, Rgba([255 - pixel[0], 255 - pixel[1], 255 - pixel[2], pixel[3]]));
    //     }
    // }

    // new_img.save("E:\\rust\\image_inverted.jpg").expect("Failed to save image");
}
