use crate::consts::DICOM_TAGS;
use std::collections::HashMap;
use std::path::PathBuf;
use dicom::object::open_file;
use dicom::pixeldata::{PixelDecoder};
use dicom::object::FileDicomObject;
use dicom::core::{Tag, VR};
use dicom::object::mem::InMemDicomObject;
use dicom::object::StandardDataDictionary;
use dicom::core::dictionary::{DataDictionary, DataDictionaryEntry};
use chrono::NaiveDateTime;

/// Represents the various data types that can be extracted from DICOM tags.
/// 
/// This enum provides a unified way to handle different DICOM value representations,
/// including strings, numeric types, dates, and times. Each variant corresponds
/// to a specific DICOM Value Representation (VR) type.
#[derive(Debug, Clone)]
pub enum Value {
    /// String values from DICOM tags (PN, LO, SH, CS, UI VRs)
    String(String),
    /// Integer values from DICOM tags (IS VR)
    Integer(i64),
    /// Floating-point values from DICOM tags (DS, FL, FD VRs)
    Float(f64),
    /// Unsigned integer values from DICOM tags (US, UL VRs)
    UnsignedInteger(u64),
    /// Date values from DICOM tags (DA, DT VRs)
    Date(chrono::NaiveDateTime),
    /// Time values from DICOM tags (TM VR)
    Time(chrono::NaiveTime),
}

/// Implements display formatting for Value enum variants.
/// 
/// This allows Value instances to be printed in a human-readable format.
/// Each variant is formatted appropriately for its data type.
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::UnsignedInteger(ui) => write!(f, "{}", ui),
            Value::Date(d) => write!(f, "{}", d),
            Value::Time(t) => write!(f, "{}", t),
        }
    }
}

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
/// The pixel data is stored as a multi-dimensional array of 16-bit unsigned integers,
/// which is the typical format for medical imaging data.
#[allow(dead_code)]
pub struct DicomData {
    pub path: PathBuf,
    pub tags: HashMap<String, Value>,
    pub pixel_data: Option<ndarray::Array<u16, ndarray::IxDyn>>,
}
    

/// Extracts specific DICOM tags from a DICOM object and converts them to appropriate Rust types.
/// 
/// This function processes a list of DICOM tags and extracts their values from the provided
/// DICOM object. It uses the Value Representation (VR) of each tag to determine the appropriate
/// parsing method and data type conversion.

fn extract_dicom_tags(dicom: &FileDicomObject<InMemDicomObject>, tags: &[Tag]) -> HashMap<String, Value> {
    // extract dicom tags. use VR to determine how to parse the value.
    let mut tags_map: HashMap<String, Value> = HashMap::new();

    for tag in tags {
        // Use the standard dictionary to get the tag name
        let tag_name = StandardDataDictionary.by_tag(*tag)
            .map(|entry| entry.alias())
            .unwrap_or("Unknown Tag");
        
        match dicom.element(*tag) {
            Ok(elem) => {
                let vr = elem.vr(); 
                let value = match vr {
                    VR::PN | VR::LO | VR::SH | VR::CS | VR::UI => {
                        match elem.to_str() {
                            Ok(s) => Value::String(s.into_owned()),
                            Err(_) => Value::String("<parse error>".to_string()),
                        }
                    },
                    VR::DA => {
                        // Date format: YYYYMMDD
                        match elem.to_str() {
                            Ok(s) => {
                                let date_str = s.as_ref();
                                // First try to parse as just a date
                                match chrono::NaiveDate::parse_from_str(date_str, "%Y%m%d") {
                                    Ok(date) => {
                                        let dt = date.and_hms_opt(0, 0, 0).unwrap();
                                        Value::Date(dt)
                                    },
                                    Err(_) => {
                                        Value::String(s.into_owned())
                                    }
                                }
                            },
                            Err(_) => Value::String("<parse error>".to_string()),
                        }
                    },
                    VR::TM => {
                        // Time format: HHMMSS.FFFFFF
                        match elem.to_str() {
                            Ok(s) => {
                                let time_str = s.as_ref();
                                // Try different time formats
                                let formats = ["%H%M%S", "%H%M%S.%f", "%H%M%S.%3f", "%H%M%S.%6f"];
                                let mut result = Value::String(s.clone().into_owned());
                                
                                for format in formats.iter() {
                                    match chrono::NaiveTime::parse_from_str(time_str, format) {
                                        Ok(time) => {
                                            result = Value::Time(time);
                                            break;
                                        },
                                        Err(_) => continue,
                                    }
                                }
                                
                                result
                            },
                            Err(_) => Value::String("<parse error>".to_string()),
                        }
                    },
                    VR::DT => {
                        // DateTime format: YYYYMMDDHHMMSS.FFFFFF
                        match elem.to_str() {
                            Ok(s) => {
                                let dt_str = s.as_ref();
                                // Try different formats
                                let formats = ["%Y%m%d%H%M%S", "%Y%m%d%H%M%S%.f"];
                                let mut result = Value::String(s.clone().into_owned());
                                
                                for format in formats.iter() {
                                    match NaiveDateTime::parse_from_str(dt_str, format) {
                                        Ok(dt) => {
                                            result = Value::Date(dt);
                                            break;
                                        },
                                        Err(_) => continue,
                                    }
                                }
                                
                                result
                            },
                            Err(_) => Value::String("<parse error>".to_string()),
                        }
                    },
                    VR::US => {
                        match elem.to_int::<u16>() {
                            Ok(v) => Value::UnsignedInteger(v as u64),
                            Err(_) => Value::String("<parse error>".to_string()),
                        }
                    },
                    VR::UL => {
                        match elem.to_int::<u32>() {
                            Ok(v) => Value::UnsignedInteger(v as u64),
                            Err(_) => Value::String("<parse error>".to_string()),
                        }
                    },
                    VR::IS => {
                        match elem.to_int::<i32>() {
                            Ok(v) => Value::Integer(v as i64),
                            Err(_) => Value::String("<parse error>".to_string()),
                        }
                    },
                    VR::DS | VR::FL | VR::FD => {
                        match elem.to_float64() {
                            Ok(v) => Value::Float(v),
                            Err(_) => Value::String("<parse error>".to_string()),
                        }
                    },
                    _ => {
                        // fallback to raw string if we can't guess
                        match elem.to_str() {
                            Ok(s) => Value::String(s.into_owned()),
                            Err(_) => Value::String("<binary/unsupported>".to_string()),
                        }
                    }
                };

                tags_map.insert(tag_name.to_string(), value);
            }
            Err(e) => {
                tags_map.insert(tag_name.to_string(), Value::String(format!("{}", e)));
            }
        }
    }

    tags_map
}

/// Extracts and decodes pixel data from a DICOM object.
/// 
/// This function attempts to extract the pixel data from a DICOM file and convert it
/// to a multi-dimensional array format suitable for image processing and analysis.
/// The pixel data is typically stored as 16-bit unsigned integers (u16) which is
/// the standard format for medical imaging data like CT scans and X-rays.
/// 
/// The function performs the following steps:
/// 1. Decodes the raw pixel data from the DICOM object
/// 2. Converts the decoded data to a multi-dimensional array
/// 3. Transforms the data into the ndarray crate format for easier manipulation
/// 4. Provides detailed logging about the array shape and dimensions
fn extract_dicom_pixel_data(dicom: &FileDicomObject<InMemDicomObject>) -> Option<ndarray::Array<u16, ndarray::IxDyn>> {
    if let Ok(pixel_data) = dicom.decode_pixel_data() {
        // Convert to an n-dimensional array of u16 (typical for CT pixel values)
        match pixel_data.to_ndarray::<u16>() {
            Ok(array) => {
                println!("Pixel array shape: {:?}", array.shape());
                let ndim = array.ndim();
                println!("Array has {} dimensions with shape {:?}", ndim, array.shape());
                // Convert dicom's ndarray to main ndarray crate
                let vec_data: Vec<u16> = array.iter().cloned().collect();
                let shape = array.shape().to_vec();
                Some(ndarray::Array::from_vec(vec_data).into_shape_with_order(shape).unwrap().into_dyn())
            },
            Err(_) => {
                println!("Failed to convert pixel data to ndarray");
                None
            }
        }
    } else {
        println!("Failed to decode pixel data");
        None
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

