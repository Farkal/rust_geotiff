use rust_geotiff::decoder::*;
use rust_geotiff::encoder::TiffEncoder;
use rust_geotiff::tags;
use tiff::encoder::colortype;
// use tiff::tags::{CompressionMethod};
use std::{
    fs::File,
    io::{Seek, SeekFrom},
};

#[test]
fn test_encoder() {
    let mut file = std::io::Cursor::new(Vec::new());
    let mut image_data = Vec::new();
    for x in 0..100 {
        for y in 0..100u8 {
            let val = x + y;
            image_data.push(val);
            image_data.push(val);
            image_data.push(val);
        }
    }
    let image_data = &image_data[..];
    {
        let mut encoder = TiffEncoder::new(&mut file).expect("Cannot create decoder");
        let mut image = encoder.new_image::<colortype::RGB8>(100, 100).unwrap();
        image
            .encoder()
            .write_tag(tags::GDALNODATA, "-5999")
            .unwrap();
        image.write_data(&image_data).unwrap();
        // encoder.encoder.write_image(100, 100, &image_data, colortype::RGB8, CompressionMethod::None, vec![(tags::GDALNODATA, Value::Str("-5999".into()))]).unwrap();
    }
    {
        file.seek(SeekFrom::Start(0)).unwrap();
        let mut decoder = Decoder::new(&mut file).expect("Cannot create decoder");
        // println!("{:#?}", decoder);
        println!("HERE !");
        let no_data = decoder
            .get_tag(tags::GDALNODATA)
            .expect("Failed to get tag");
        println!("{:?}", no_data);
    }
}
#[test]
fn test_decode() {
    let img_file = File::open("resources/zh_dem_25.tif").expect("Cannot find test image!");
    let mut decoder = Decoder::new(img_file).expect("Cannot create decoder");
    println!("{:#?}", decoder);
    let no_data = decoder
        .get_tag(tags::GDALNODATA)
        .expect("Failed to get tag");
    println!("{:?}", no_data);
}

// #[test]
// fn test_load_2() {
//     match TIFF::open("resources/zh_dem_25.tif") {
//         Ok(x) => {
//             assert_eq!(x.image_data.len(), 366);
//             assert_eq!(x.image_data[0].len(), 399);

//             assert_eq!(x.get_value_at(0, 0), 551);
//             assert_eq!(x.get_value_at(45, 67), 530);
//             assert_eq!(x.get_value_at(142, 325), 587);
//         },
//         Err(e) => println!("File I/O Error: {:?}", e),
//     }
// }

// // TODO Not supported yet, as this uses TileByteCounts instead of StripByteCounts.
// //#[test]
// fn test_load_3() {
//     match TIFF::open("resources/large_tif/DEM_ZH.tif") {
//         Ok(x) => {
//             assert_eq!(x.image_data.len(), 366);
//             assert_eq!(x.image_data[0].len(), 399);

//             assert_eq!(x.get_value_at(0, 0), 551);
//             assert_eq!(x.get_value_at(45, 67), 530);
//             assert_eq!(x.get_value_at(142, 325), 587);
//         },
//         Err(e) => println!("File I/O Error: {:?}", e),
//     }
// }
