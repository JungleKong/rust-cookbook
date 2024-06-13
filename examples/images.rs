use image::{self, DynamicImage, ImageBuffer};
use png;

use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn main() {

    
    // 编译期读取，camera_0.raw 1920x1280 (not certain `UYUV or YUYV`)
    const IMAGE_BYTE: &'static [u8] = include_bytes!("../../assets/images/camera_0.raw");
    const _IMAGE_DATA: &'static [u8] = include_bytes!("../../assets/images/test.png");
    
    let stage = Instant::now();
    // for _ in 0..1000 {
        let _v: Vec<u8> = IMAGE_BYTE.into();
    // }

    // image::load_from_memory(IMAGE_DATA).unwrap();

    let stage1 = Instant::now();
    println!("stage1: {:?}", stage1.duration_since(stage));


    let stage2 = Instant::now();
    println!("stage2: {:?}", stage2.duration_since(stage1));

    // 将buffer保存为图片
    match image::save_buffer("../assets/images/test.png", IMAGE_BYTE, 1920, 1280, image::ColorType::L16) {
        Ok(_) => println!("save success"),
        Err(e) => println!("save error: {}", e),
    }

    let stage3 = Instant::now();
    println!("stage3: {:?}", stage3.duration_since(stage2));

    // 从raw中加载一张图片，因为raw不包含宽高信息，所以需要手动指定宽高
    // 同时因为raw的格式为UYUV，image库不支持，所以需要转换为LumaA，但是结果不正确，todo!()
    let a = ImageBuffer::<image::LumaA<u8>, Vec<u8>>::from_raw(1920, 1280, IMAGE_BYTE.into()).unwrap();
    
    let stage4 = Instant::now();
    println!("stage4: {:?}", stage4.duration_since(stage3));
    
    let img = DynamicImage::ImageLumaA8(a.clone());

    let stage5 = Instant::now();
    println!("stage5: {:?}", stage5.duration_since(stage4));

    img.save("../assets/images/test2.png").unwrap();

    let stage6 = Instant::now();
    println!("stage6: {:?}", stage6.duration_since(stage5));

    // 直接打开图片
    let img = image::open("../assets/images/test.png").unwrap();

    let stage7 = Instant::now();
    println!("stage7: {:?}", stage7.duration_since(stage6));

    // 保存图片buffer为raw，保存结果不包含宽高信息
    let mut file = File::create("../assets/images/camera.raw").unwrap();
    file.write_all(img.as_bytes()).unwrap();


    let png_stage1 = Instant::now();
    println!("png_stage1: {:?}", png_stage1.duration_since(stage7));

    let _decoder = png::Decoder::new(File::open("../assets/images/test.png").unwrap()).read_info().unwrap();


}
