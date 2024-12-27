use sha1::{Sha1, Digest};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::bufread::ZlibDecoder;
use std::io::{self, Read, Write};
use std::fs::{self, OpenOptions};
use std::path::Path;

pub fn calculate_sha1(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data); 
    let hash_result = hasher.finalize();
    format!("{:x}", hash_result)
}


fn compress_object(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish()
}

fn decompress_object(compressed_data: &[u8]) -> io::Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(compressed_data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}

pub fn create_object(objects_dir: &Path, file_data: &[u8], hash: &str){
    let object_header = format!("blob {}\0", file_data.len());
    let mut object_data = Vec::new();
    object_data.extend_from_slice(object_header.as_bytes());
    object_data.extend_from_slice(file_data);

    let compressed_object = compress_object(&object_data).expect("Error compressing object");

    let object_dir = objects_dir.join(&hash[..2]);
    let object_file = object_dir.join(&hash[2..]);

    fs::create_dir_all(&object_dir).expect("Unable to create new object directory");
    let mut object_file = OpenOptions::new().write(true).create(true).open(object_file).expect("Error opening objevt file");
    object_file.write_all(&compressed_object).expect("Error writing to object directory");
}

pub fn retrieve_object(objects_dir: &Path, hash: &str) -> io::Result<Vec<u8>> {
    let object_dir = objects_dir.join(&hash[..2]); 
    let object_file_path = object_dir.join(&hash[2..]); 
   
    let compressed_data = fs::read(object_file_path)?;
    let mut decompressed_data = decompress_object(&compressed_data)?;

    if let Some(index) = decompressed_data.iter().position(|&x| x == 0) {
        Ok(decompressed_data.split_off(index + 1))
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid object data"))
    }
}