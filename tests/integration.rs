use std::fs::File;
use rust_geotiff::decoder::Decoder;

#[test]
fn test_load() {
    let img_file = File::open("resources/marbles.tif").expect("Cannot find test image!");
    let decoder = Decoder::new(img_file).expect("Cannot create decoder");
    println!("{:?}", decoder)
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
