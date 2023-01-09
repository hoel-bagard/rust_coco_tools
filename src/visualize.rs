pub mod bbox;
pub mod segmentation;

use crate::annotations::load_coco::HashmapDataset;
use crate::errors;
use image::io::Reader as ImageReader;
use rand::Rng;
use show_image::create_window;
use std::path::Path;

/// # Panics
///
/// Will panic if it cannot read the image file corresponding to the `img_id`.
///
/// # Errors
///
/// Will return `Err` if `img_id` is not present in the dataset.
pub fn visualize_img(
    dataset: &HashmapDataset,
    image_folder: &String,
    img_id: u32,
) -> Result<(), errors::MissingIdError> {
    let sample_path = Path::new(image_folder).join(&dataset.get_img(img_id)?.file_name);

    let mut img = ImageReader::open(&sample_path)
        .unwrap_or_else(|error| {
            panic!(
                "Could not open the image {}: {:?}",
                sample_path.display(),
                error
            );
        })
        .decode()
        .unwrap_or_else(|error| {
            panic!(
                "Could not decode the image {}: {:?}",
                sample_path.display(),
                error
            );
        })
        .into_rgb8();

    let mut rng = rand::thread_rng();
    for ann in dataset.get_img_anns(img_id)? {
        let color = image::Rgb([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()]);
        bbox::draw_bbox(&mut img, &ann.bbox, color);
        let mask = segmentation::Mask::from(&ann.segmentation);
        segmentation::draw_mask(&mut img, &mask, color);
    }

    // Use show_image or viuer here.
    // img.save("outputs/out.jpg").unwrap_or_else(|error| {
    //     panic!("Could not save the image: {:?}", error);
    // });

    let window = create_window("image", Default::default())
        .map_err(|e| e.to_string())
        .unwrap();
    window
        .set_image("aaa", img)
        .map_err(|e| e.to_string())
        .unwrap();

    Ok(())
}
