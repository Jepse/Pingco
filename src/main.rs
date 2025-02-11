use std::env;
use std::fs;
use std::path::Path;
use image::{ImageFormat};
use image::codecs::ico::{IcoEncoder, IcoFrame};

// Output files will be saved in the same folder as the input file in the following format:
const ICO_SIZES: [u32; 10] = [16, 20, 24, 32, 40, 48, 64, 96, 128, 256];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return; 
    }
    
    let input_path = Path::new(&args[1]);
    if !input_path.exists() {
        return; 
    }
    
    if let Some(extension) = input_path.extension().and_then(|ext| ext.to_str()) {
        match extension.to_lowercase().as_str() {
            "png" => convert_png_to_ico(input_path),
            "ico" => convert_ico_to_png(input_path),
            _ => return, 
        }
    }
}

fn convert_png_to_ico(input_path: &Path) {
    let img = match image::open(input_path) {
        Ok(img) => img,
        Err(_) => return, 
    };
    
    let output_path = input_path.with_file_name(
        input_path.file_stem().unwrap().to_string_lossy().to_string() + ".ico"
    );
    
    let frames: Vec<IcoFrame> = ICO_SIZES.iter().filter_map(|&size| {
        let resized = img.resize_exact(size, size, image::imageops::FilterType::Nearest);
        IcoFrame::as_png(resized.to_rgba8().as_raw(), size, size, resized.color().into()).ok()
    }).collect();
    
    let file = match fs::File::create(&output_path) {
        Ok(f) => f,
        Err(_) => return, 
    };
    
    let encoder = IcoEncoder::new(file);
    let _ = encoder.encode_images(&frames); 
}

fn convert_ico_to_png(input_path: &Path) {
    let img = match image::open(input_path) {
        Ok(img) => img,
        Err(_) => return, 
    };
    
    let output_path = input_path.with_file_name(
        input_path.file_stem().unwrap().to_string_lossy().to_string() + ".png"
    );
    
    let _ = img.save_with_format(output_path, ImageFormat::Png);
}
