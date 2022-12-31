use std::{path::Path, fs::File, io::{Read, BufReader, self}, time::Duration};

pub fn read_zipped_xml_file(path: &Path) -> io::Result<String> {
    if path.ends_with(".xml") {
        let mut file_content: String = String::new();
        File::open(path)?.read_to_string(&mut file_content)?;
        Ok(file_content)
    }   
    else { // .xml.gz file
        let mut file_content: String = String::new();
        let decoder = flate2::read::GzDecoder::new(File::open(path)?);
        BufReader::new(decoder).read_to_string(&mut file_content)?;
        Ok(file_content)
    }
}

pub fn measure_time<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = std::time::Instant::now();
    let result = f();
    (result, start.elapsed())
}

pub fn print_time<T>(
    f: impl FnOnce() -> T,
    print: impl FnOnce(Duration),
) -> T {
    let (result, time) = measure_time(f);
    print(time);
    result
}

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    *t == Default::default()
}
