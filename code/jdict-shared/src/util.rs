use std::{path::Path, fs::File, io::{Read, BufReader, self}, time::Duration};

pub fn load_gzip_file(file: File) -> io::Result<String> {
    let mut file_content = String::new();
    let decoder = flate2::read::GzDecoder::new(file);
    BufReader::new(decoder).read_to_string(&mut file_content)?;
    Ok(file_content)
}

pub fn load_file(mut file: File) -> io::Result<String> {
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    Ok(file_content)
}

pub fn decompress(data: &[u8]) -> io::Result<String> {
    let decoder = flate2::read::GzDecoder::new(data);
    let mut file_content = String::new();
    BufReader::new(decoder).read_to_string(&mut file_content)?;
    Ok(file_content)
}

pub fn read_file(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;

    if path.ends_with(".xml") {
        load_file(file)
    }
    else { // .xml.gz file
        load_gzip_file(file)
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
