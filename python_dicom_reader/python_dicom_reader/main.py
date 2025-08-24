#!/usr/bin/env python3
"""
Fast DICOM Reader - Python Translation
Translated from Rust implementation to provide equivalent functionality using pydicom.
"""

import os
import argparse
import sys
from pathlib import Path
from typing import List, Dict, Optional, Any
from dataclasses import dataclass
import numpy as np
from tqdm import tqdm

try:
    import pydicom
    from pydicom import Dataset
    from pydicom.errors import InvalidDicomError
except ImportError:
    print("Error: pydicom is required. Install with: pip install pydicom")
    sys.exit(1)

# DICOM Tags Constants (translated from consts.rs)
DICOM_TAGS = [
    (0x0020, 0x000D),  # STUDY_INSTANCE_UID
    (0x0020, 0x000E),  # SERIES_INSTANCE_UID
    (0x0008, 0x0018),  # SOP_INSTANCE_UID
    (0x0020, 0x0013),  # INSTANCE_NUMBER
    (0x0008, 0x0060),  # MODALITY
    (0x0008, 0x1030),  # STUDY_DESCRIPTION
    (0x0008, 0x103E),  # SERIES_DESCRIPTION
    (0x0018, 0x0015),  # BODY_PART_EXAMINED
    (0x0018, 0x0022),  # SCAN_OPTIONS
    (0x0018, 0x0050),  # SLICE_THICKNESS
    (0x0040, 0x0254),  # PERFORMED_PROCEDURE_STEP_DESCRIPTION
    (0x0028, 0x1050),  # WINDOW_CENTER
    (0x0028, 0x1051),  # WINDOW_WIDTH
    (0x0020, 0x0032),  # IMAGE_POSITION_PATIENT
    (0x0018, 0x1030),  # PROTOCOL_NAME
    (0x0028, 0x1052),  # RESCALE_INTERCEPT
    (0x0028, 0x1053),  # RESCALE_SLOPE
    (0x0008, 0x0080),  # INSTITUTION_NAME
    (0x0008, 0x0081),  # INSTITUTION_ADDRESS
    (0x0028, 0x0030),  # PIXEL_SPACING
    (0x0010, 0x0010),  # PATIENT_NAME
    (0x0010, 0x0030),  # PATIENT_BIRTH_DATE
    (0x0010, 0x0040),  # PATIENT_SEX
    (0x0010, 0x1010),  # PATIENT_AGE
    (0x0010, 0x0020),  # PATIENT_ID
    (0x0008, 0x0020),  # STUDY_DATE
    (0x0008, 0x0021),  # SERIES_DATE
    (0x0008, 0x0022),  # ACQUISITION_DATE
    (0x0008, 0x0023),  # CONTENT_DATE
    (0x0008, 0x0012),  # INSTANCE_CREATION_DATE
    (0x0008, 0x0030),  # STUDY_TIME
    (0x0008, 0x0031),  # SERIES_TIME
    (0x0008, 0x0032),  # ACQUISITION_TIME
    (0x0008, 0x0033),  # CONTENT_TIME
    (0x0008, 0x0013),  # INSTANCE_CREATION_TIME
    (0x0008, 0x0050),  # ACCESSION_NUMBER
    (0x0008, 0x0201),  # TIMEZONE_OFFSET_FROM_UTC
    (0x0008, 0x0090),  # REFERRING_PHYSICIAN_NAME
    (0x0020, 0x0037),  # IMAGE_ORIENTATION_PATIENT
    (0x0028, 0x0004),  # PHOTOMETRIC_INTERPRETATION
    (0x0032, 0x1060),  # REQUESTED_PROCEDURE_DESCRIPTION
    (0x0020, 0x0011),  # SERIES_NUMBER
    (0x0018, 0x0088),  # SPACING_BETWEEN_SLICES
    (0x0008, 0x0070),  # MANUFACTURER
    (0x0008, 0x1090),  # MANUFACTURER_MODEL_NAME
    (0x0018, 0x5100),  # PATIENT_POSITION
    (0x0018, 0x1210),  # CONVOLUTION_KERNEL
    (0x0018, 0x1100),  # RECONSTRUCTION_DIAMETER
    (0x0018, 0x0060),  # KVP
    (0x0028, 0x0008),  # NUMBER_OF_FRAMES
    (0x5200, 0x9230),  # PER_FRAME_FUNCTIONAL_GROUPS_SEQUENCE
    (0x5200, 0x9229),  # SHARED_FUNCTIONAL_GROUPS_SEQUENCE
    (0x0020, 0x9116),  # PLANE_ORIENTATION_SEQUENCE
    (0x0020, 0x9113),  # PLANE_POSITION_SEQUENCE
    (0x0008, 0x0008),  # IMAGE_TYPE
    (0x0028, 0x0011),  # COLUMNS
    (0x0028, 0x0010),  # ROWS
]


@dataclass
class DicomData:
    """
    Represents the complete data extracted from a DICOM file.

    This class contains all the relevant information from a DICOM file,
    including the file path, extracted tag values, and pixel data if available.
    The pixel data is stored as a numpy array of 32-bit signed integers,
    which preserves the original values including negative numbers for CT scans.
    """

    path: Path
    tags: Dict[str, str]
    pixel_data: Optional[np.ndarray]


def get_dicom_paths_from_folder(folder_path: str) -> List[Path]:
    """
    Get all DICOM file paths from a folder recursively.
    Filters out .DS_Store files.
    Translated from os_utils.rs
    """
    dicom_paths = []

    for root, dirs, files in os.walk(folder_path):
        for file in files:
            if file != ".DS_Store":
                file_path = Path(root) / file
                dicom_paths.append(file_path)

    return dicom_paths


def read_dicom_file(filepath: str) -> Dataset:
    """
    Read a DICOM file using pydicom.
    Translated from dicom_utils.rs
    """
    try:
        dicom_obj = pydicom.dcmread(filepath)
        return dicom_obj
    except (InvalidDicomError, Exception) as e:
        print(f"Failed to open DICOM file: {filepath}, {e}")
        raise e


def extract_dicom_tags(dicom: Dataset, tags: List[tuple]) -> Dict[str, str]:
    """
    Extracts specific DICOM tags from a DICOM dataset and converts them to strings.

    This function processes a list of DICOM tags and extracts their values from the provided
    DICOM dataset. All values are converted to strings regardless of their original DICOM type.
    Translated from dicom_utils.rs
    """
    tags_map = {}

    for tag in tags:
        try:
            # Get the tag name from pydicom's keyword mapping
            keyword = pydicom.datadict.keyword_for_tag(tag)
            tag_name = keyword if keyword else f"Tag_{tag[0]:04X}_{tag[1]:04X}"

            if tag in dicom:
                element = dicom[tag]
                # Convert element value to string
                if hasattr(element, "value"):
                    if isinstance(element.value, (list, pydicom.multival.MultiValue)):
                        # Handle multi-valued elements
                        tags_map[tag_name] = str(list(element.value))
                    else:
                        tags_map[tag_name] = str(element.value)
                else:
                    tags_map[tag_name] = str(element)
            else:
                tags_map[tag_name] = "Tag not found"

        except Exception as e:
            tag_name = f"Tag_{tag[0]:04X}_{tag[1]:04X}"
            tags_map[tag_name] = f"Error: {e}"

    return tags_map


def extract_dicom_pixel_data(dicom: Dataset) -> Optional[np.ndarray]:
    """
    Extracts and decodes pixel data from a DICOM dataset.

    This function attempts to extract the pixel data from a DICOM file and convert it
    to a numpy array format suitable for image processing and analysis.
    The pixel data is preserved in its original format without any value mapping.
    Translated from dicom_utils.rs
    """
    try:
        # Check if pixel data exists
        if (0x7FE0, 0x0010) not in dicom:  # PixelData tag
            return None

        # Get pixel array from pydicom
        pixel_array = dicom.pixel_array

        # Convert to int32 to match the Rust implementation
        pixel_data = pixel_array.astype(np.int32)

        return pixel_data

    except Exception as e:
        print(f"Failed to decode pixel data: {e}")
        return None


def extract_dicom_data(dicom: Dataset, path: Path) -> DicomData:
    """
    Extract complete DICOM data including tags and pixel data.
    Translated from dicom_utils.rs
    """
    tags_map = extract_dicom_tags(dicom, DICOM_TAGS)
    pixel_data = extract_dicom_pixel_data(dicom)

    return DicomData(path=path, tags=tags_map, pixel_data=pixel_data)


def process_single_dicom(path: Path) -> DicomData:
    """
    Process a single DICOM file and extract all relevant data.
    Translated from main.rs
    """
    path_str = str(path)

    try:
        dicom_obj = read_dicom_file(path_str)
    except Exception as e:
        print(f"Failed to read DICOM file {path_str}: {e}")
        raise Exception(f"Failed to read DICOM file: {e}")

    dicom_data = extract_dicom_data(dicom_obj, path)
    return dicom_data


def main():
    """
    Main function - processes DICOM files sequentially.
    Translated from main.rs but without parallelism.
    """
    parser = argparse.ArgumentParser(
        prog="Fast DICOM reader",
        description="Process DICOM files and extract metadata and pixel data",
    )

    parser.add_argument("command", choices=["read"], help="Command to execute")
    parser.add_argument(
        "-p", "--path", type=str, required=True, help="Directory path to scan for files"
    )
    parser.add_argument(
        "-t",
        "--threads",
        type=int,
        help="Number of threads to use for parallel processing (ignored in this Python version)",
    )

    args = parser.parse_args()

    if args.command == "read":
        print(f"Processing DICOM files in: {args.path}")

        # Note about threading (since user requested no parallelism)
        if args.threads:
            print(
                f"Note: Threading parameter ignored. Processing sequentially as requested."
            )

        try:
            dicom_paths = get_dicom_paths_from_folder(args.path)
            total_files = len(dicom_paths)
            print(f"Found {total_files} DICOM files to process")

            if total_files == 0:
                print("No files found to process.")
                return

            # Process files sequentially
            errors = []
            processed_count = 0

            for path in tqdm(dicom_paths, desc="Processing DICOM files", unit="file"):
                try:
                    dicom_data = process_single_dicom(path)
                    processed_count += 1

                except Exception as e:
                    errors.append(str(e))
                    print(f"  -> Error: {e}")

            print("Processing complete!")

            # Report results
            if errors:
                print(f"\nEncountered {len(errors)} errors during processing:")
                for error in errors:
                    print(f"Error: {error}")
            else:
                print("\nAll DICOM files processed successfully!")

            print(
                f"Processing completed. Total files: {total_files}, Successfully processed: {processed_count}"
            )

        except Exception as e:
            print(f"Error during processing: {e}")
            sys.exit(1)


if __name__ == "__main__":
    main()
