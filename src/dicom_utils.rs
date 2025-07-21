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

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    UnsignedInteger(u64),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::UnsignedInteger(ui) => write!(f, "{}", ui),
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

#[allow(dead_code)]
pub struct DicomData {
    pub path: PathBuf,
    pub tags: HashMap<String, Value>,
    pub pixel_data: Option<ndarray::Array<u16, ndarray::IxDyn>>,
}
    

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
                    VR::PN | VR::LO | VR::SH | VR::CS | VR::UI | VR::DA | VR::TM | VR::DT => {
                        match elem.to_str() {
                            Ok(s) => Value::String(s.into_owned()),
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
                tags_map.insert(tag_name.to_string(), Value::String(format!("<not found> ({})", e)));
            }
        }
    }

    tags_map
}

fn extract_dicom_pixel_data(dicom: &FileDicomObject<InMemDicomObject>) -> Option<ndarray::Array<u16, ndarray::IxDyn>> {
    if let Ok(pixel_data) = dicom.decode_pixel_data() {
        // Convert to an n-dimensional array of u16 (typical for CT pixel values)
        if let Ok(_array) = pixel_data.to_ndarray::<u16>() {
            println!("Pixel array shape: {:?}", _array.shape());
            let ndim = _array.ndim();
            println!("Array has {} dimensions with shape {:?}", ndim, _array.shape());
            // For now, just print the shape info without trying to access specific indices
            // to avoid dimension mismatch issues
            None // Temporarily return None until we figure out the correct type
        } else {
            println!("Failed to convert pixel data to ndarray");
            None
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

