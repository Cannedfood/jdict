use std::{path::Path, fs::File, io::{Read, BufReader}};

pub fn read_zipped_xml_file(path: &Path) -> Option<String> {
    if path.ends_with(".xml") {
        let mut file_content: String = String::new();
        File::open(path).unwrap().read_to_string(&mut file_content).unwrap();
        Some(file_content)
    }   
    else { // .xml.gz file
        let mut file_content: String = String::new();
        let decoder = flate2::read::GzDecoder::new(File::open(path).unwrap());
        BufReader::new(decoder).read_to_string(&mut file_content).unwrap();
        Some(file_content)
    }
}