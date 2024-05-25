use clap::Parser;
use fast_image_resize::images::Image;
use fast_image_resize::{IntoImageView, PixelType, ResizeOptions, Resizer};
use image::{ColorType, GenericImageView};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    target: String,

    #[arg(short, long)]
    #[clap(default_value = "output.jpg")]
    output: String,
}

fn main() {
    let args = Args::parse();
    dbg!(&args);
    let img = image::open(&args.target).unwrap();
    let (width, height) = img.dimensions();
    dbg!(width, height);

    // Calculate the target dimensions based on the 65:24 aspect ratio
    let target_ratio = 65.0 / 24.0;
    let image_ratio = width as f32 / height as f32;

    let (target_width, target_height) = if image_ratio > target_ratio {
        let target_height = height;
        let target_width = (target_height as f32 * target_ratio) as u32;
        (target_width, target_height)
    } else {
        let target_width = width;
        let target_height = (target_width as f32 / target_ratio) as u32;
        (target_width, target_height)
    };

    // Calculate the starting x and y positions for cropping
    let start_x = (width - target_width) / 2;
    let start_y = (height - target_height) / 2;

    let color_type = match img.pixel_type().unwrap() {
        PixelType::U8 => ColorType::L8,
        PixelType::U8x2 => ColorType::La8,
        PixelType::U8x3 => ColorType::Rgb8,
        PixelType::U8x4 => ColorType::Rgba8,
        PixelType::U16 => ColorType::L16,
        PixelType::U16x2 => ColorType::La16,
        PixelType::U16x3 => ColorType::Rgb16,
        PixelType::U16x4 => ColorType::Rgba16,
        _ => panic!("Unsupported type of pixels"),
    };

    // Crop the image to the target dimensions
    // target image
    let mut dst_image = Image::new(target_width, target_height, img.pixel_type().unwrap());

    // resizer
    let mut resizer = Resizer::new();
    resizer
        .resize(
            &img,
            &mut dst_image,
            &ResizeOptions::new().crop(
                start_x.into(),
                start_y.into(),
                target_width.into(),
                target_height.into(),
            ),
        )
        .unwrap();

    // Save the cropped image as a JPEG file
    image::save_buffer(
        args.output,
        dst_image.buffer(),
        dst_image.width(),
        dst_image.height(),
        color_type,
    )
    .expect("Failed to save image");

    println!("Image cropped successfully");
}
