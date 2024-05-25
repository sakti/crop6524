use clap::Parser;
use image::GenericImageView;

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
    let mut img = image::open(&args.target).unwrap();
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

    // Crop the image to the target dimensions
    let cropped_img = img.crop(start_x, start_y, target_width, target_height);

    // Save the cropped image as a JPEG file
    cropped_img.save(args.output).expect("Failed to save image");

    println!("Image cropped successfully");
}
