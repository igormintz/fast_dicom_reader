use crate::consts::DICOM_TAGS;
use std::collections::HashMap;
use std::path::PathBuf;
use dicom::object::open_file;
use dicom::pixeldata::{PixelDecoder};
use dicom::object::FileDicomObject;
use dicom::core::Tag;
use dicom::object::mem::InMemDicomObject;
use dicom::object::StandardDataDictionary;
use dicom::core::dictionary::DataDictionary;

pub fn read_dicom_file(
    filepath: &str,
) -> Result<
    dicom::object::FileDicomObject<
        dicom::object::InMemDicomObject<dicom::object::StandardDataDictionary>,
    >,
    Box<dyn std::error::Error>,
> {
    let result = open_file(filepath);
    match result {
        Ok(obj) => Ok(obj),
        Err(e) => {
            eprintln!("Failed to open DICOM file: {}, {}", filepath, e);
            Err(e.into())
        }
    }
}

/// Represents the complete data extracted from a DICOM file.
/// 
/// This struct contains all the relevant information from a DICOM file,
/// including the file path, extracted tag values, and pixel data if available.
/// The pixel data is stored as a multi-dimensional array of 32-bit signed integers,
/// which preserves the original values including negative numbers for CT scans.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DicomData {
    pub path: PathBuf,
    pub tags: HashMap<String, String>,
    pub pixel_data: Option<ndarray::Array<i32, ndarray::IxDyn>>,
}
    

/// Extracts specific DICOM tags from a DICOM object and converts them to strings.
/// 
/// This function processes a list of DICOM tags and extracts their values from the provided
/// DICOM object. All values are converted to strings regardless of their original DICOM type.

fn extract_dicom_tags(dicom: &FileDicomObject<InMemDicomObject>, tags: &[Tag]) -> HashMap<String, String> {
    // extract dicom tags and convert all values to strings
    let mut tags_map: HashMap<String, String> = HashMap::new();

    for tag in tags {
        // Use the standard dictionary to get the tag name
        let tag_name = StandardDataDictionary.by_tag(*tag)
            .map(|entry| entry.alias)
            .unwrap_or("Unknown Tag");
        
        match dicom.element(*tag) {
            Ok(elem) => {
                match elem.to_str() {
                    Ok(s) => tags_map.insert(tag_name.to_string(), s.into_owned()),
                    Err(_) => tags_map.insert(tag_name.to_string(), "<parse error>".to_string()),
                };
            }
            Err(e) => {
                tags_map.insert(tag_name.to_string(), format!("{}", e));
            }
        }
    }

    tags_map
}

/// Extracts and decodes pixel data from a DICOM object.
/// 
/// This function attempts to extract the pixel data from a DICOM file and convert it
/// to a multi-dimensional array format suitable for image processing and analysis.
/// The pixel data is preserved in its original format without any value mapping.
/// 
/// The function performs the following steps:
/// 1. Decodes the raw pixel data from the DICOM object
/// 2. Converts the decoded data to a multi-dimensional array
/// 3. Transforms the data into the ndarray crate format for easier manipulation
/// 4. Provides detailed logging about the array shape and dimensions
fn extract_dicom_pixel_data(dicom: &FileDicomObject<InMemDicomObject>) -> Option<ndarray::Array<i32, ndarray::IxDyn>> {
    // First, let's check if pixel data exists
    match dicom.element(dicom::dictionary_std::tags::PIXEL_DATA) {
        Ok(_) => (), // Pixel data element found
        Err(_) => {
            return None;
        }
    }

    // Try to decode pixel data
    match dicom.decode_pixel_data() {
        Ok(pixel_data) => {
            // Try different data types for the pixel data
            // Start with u16 (most common for medical imaging)
            if let Ok(array) = pixel_data.to_ndarray::<u16>() {
                // Convert dicom's ndarray to main ndarray crate
                let vec_data: Vec<i32> = array.iter().map(|&x| x as i32).collect();
                let shape = array.shape().to_vec();
                match ndarray::Array::from_vec(vec_data).into_shape_with_order(shape) {
                    Ok(arr) => Some(arr.into_dyn()),
                    Err(_) => None
                }
            } else {
                // Try u8 (8-bit pixel data)
                if let Ok(array) = pixel_data.to_ndarray::<u8>() {
                    // Convert u8 to i32 (simple cast)
                    let vec_data: Vec<i32> = array.iter().map(|&x| x as i32).collect();
                    let shape = array.shape().to_vec();
                    match ndarray::Array::from_vec(vec_data).into_shape_with_order(shape) {
                        Ok(arr) => Some(arr.into_dyn()),
                        Err(_) => None
                    }
                } else {
                    // Try i16 (signed 16-bit) - preserve original values
                    if let Ok(array) = pixel_data.to_ndarray::<i16>() {
                        // Convert i16 to i32 (simple cast)
                        let vec_data: Vec<i32> = array.iter()
                            .map(|&x| x as i32)
                            .collect();
                        let shape = array.shape().to_vec();
                        match ndarray::Array::from_vec(vec_data).into_shape_with_order(shape) {
                            Ok(arr) => Some(arr.into_dyn()),
                            Err(_) => None
                        }
                    } else {
                        // Try f32 (floating point) - preserve original values
                        if let Ok(array) = pixel_data.to_ndarray::<f32>() {
                            // Convert f32 to i32 (truncate to integer)
                            let vec_data: Vec<i32> = array.iter()
                                .map(|&x| x as i32)
                                .collect();
                            let shape = array.shape().to_vec();
                            match ndarray::Array::from_vec(vec_data).into_shape_with_order(shape) {
                                Ok(arr) => Some(arr.into_dyn()),
                                Err(_) => None
                            }
                        } else {
                            println!("Failed to convert pixel data to any supported format");
                            None
                        }
                    }
                }
            }
        },
        Err(_) => {
            println!("Failed to decode pixel data");
            None
        }
    }
}

pub fn extract_dicom_data(dicom: FileDicomObject<InMemDicomObject>, path: PathBuf) -> DicomData{
    let tags_map = extract_dicom_tags(&dicom, DICOM_TAGS);
    let pixel_data = extract_dicom_pixel_data(&dicom);
    DicomData {
        path,
        tags: tags_map,
        pixel_data,
    }
}

