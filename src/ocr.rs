// Reads the url
use std::ops::{Deref, Drop};
use std::env;
use std::fs::{File, remove_file};
use std::path::{Path, PathBuf};
use std::io::{self, BufRead, Read, BufWriter, Write};
use std::process::{Command, ChildStderr, Stdio};
use std::string::FromUtf8Error;
use reqwest::{get, Response};
use time::get_time;

fn execute(path: String, lang: Option<&String>) -> Option<String> {
    let mut cmd = Command::new("tesseract");
    if lang.is_some(){
        let lang_str : String = lang.unwrap().to_owned();
        cmd.arg("-l").arg(lang_str.as_str());
    }
    let output = cmd.arg(path).arg("stdout").output().unwrap();
    match String::from_utf8(output.stdout) {
        Ok(st) => Some(st),
        Err(e) => None,
    }
}

struct TempImageFile {
    filename: String,
    file: File,
    path: String,
}

impl TempImageFile {
    fn new(prefix: &str, suffix: &str) -> Self {
        let mut filename = String::new();
        let timestamp: String = get_time().sec.to_string();
        filename.push_str(prefix);
        filename.push_str(&timestamp[..]);
        filename.push_str(suffix);
        let mut abspath = String::new();
        abspath.push_str(env::temp_dir()
                             .into_os_string()
                             .into_string()
                             .unwrap()
                             .deref());
        abspath.push_str("/");
        abspath.push_str(filename.clone().deref());
        debug!("creating a filename {:?}", abspath.as_str());
        TempImageFile {
            filename: filename.clone(),
            file: File::create(&abspath[..]).unwrap(),
            path: abspath,
        }
    }

    fn into_file(&mut self) -> &mut File {
        &mut self.file
    }

    fn path(&self) -> String {
        self.path.clone()
    }
}

impl Drop for TempImageFile {
    fn drop(&mut self) -> () {
        debug!("deleting file {:?}", self.path);
        remove_file(Path::new(&self.path[..]));
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImageFormat {
    PNG,
    JPG,
    TIFF,
    BMP,
    GIF,
}

impl<'a> From<&'a str> for ImageFormat {
    fn from(r: &'a str) -> Self {
        match r {
            "png" => ImageFormat::PNG,
            "jpg" => ImageFormat::JPG,
            "tiff" => ImageFormat::TIFF,
            "bmp" => ImageFormat::BMP,
            "gif" => ImageFormat::GIF,
            _ => ImageFormat::JPG,
        }
    }
}

impl<'a> From<ImageFormat> for &'a str {
    fn from(r: ImageFormat) -> Self {
        match r {
            ImageFormat::PNG => "png",
            ImageFormat::JPG => "jpg",
            ImageFormat::TIFF => "tiff",
            ImageFormat::BMP => "bmp",
            ImageFormat::GIF => "gif",
            _ => "jpg",
        }
    }
}

pub struct ImageReader<T: Read>{
    reader: T,
    format: ImageFormat,
}

impl<T: Read> ImageReader<T> {
    pub fn tempfile(&self) -> TempImageFile {
        let mut suf: String = String::new();
        suf.push_str(".");
        suf.push_str(self.format.into());
        TempImageFile::new("tess_", &suf[..])
    }

    pub fn new(R: T, format: ImageFormat) -> Self {
        ImageReader { reader: R, format }
    }


    pub fn text(&mut self, lang: Option<&String>) -> Option<String> {
        let mut temporary = self.tempfile();
        let path = temporary.path();
        let mut writer = BufWriter::new(temporary.into_file());
        let mut buff: Vec<u8> = Vec::new();
        let amt = self.reader.read_to_end(&mut buff).unwrap();
        writer.write_all(buff.as_slice());
        writer.flush();
        let tfile = writer.into_inner().unwrap();
        execute(path, lang)
    }
}

#[derive(Debug, Clone)]
pub struct ImageBuilder {
    url: String,
}

impl ImageBuilder {
    pub fn from_url(url: &str) -> Self {
        ImageBuilder { url: url.to_string() }
    }

    pub fn format(&self) -> ImageFormat {
        let item: &str = self.url.split(".").last().unwrap();
        item.into()
    }

    pub fn reader(&self) -> ImageReader<Response> {
        let response = get(&self.url[..]).unwrap();
        ImageReader::new(response, self.format())
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_image_builder_format() {
        let image = ImageBuilder::from_url("https://www.pyimagesearch.com/wp-content/uploads/2017/06/example_01.png");
        assert_eq!(image.format(), ImageFormat::PNG);
    }

    #[test]
    fn test_image_reader_text() {
        let image = ImageBuilder::from_url("https://i.stack.imgur.com/t3qWG.png");
        let mut reader = image.reader();
        let text = reader.text(None).unwrap();
        assert_ne!(text.len(), 0);
    }
}