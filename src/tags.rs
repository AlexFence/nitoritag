use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use id3;
use taglib;

pub enum Format {
    MP3,
    OGG,
    FLAC,
}

impl Format {
    pub fn get_format(path: PathBuf) -> Option<Self> {
        match path.extension() {
            Some(ext) => {
                match ext.to_str().unwrap() {
                    "mp3" => Some(Format::MP3),
                    "ogg" => Some(Format::OGG),
                    "flac" => Some(Format::FLAC),
                    _ => None,
                }
            }
            None => None,
        }
    }
}

#[derive(Clone)]
pub struct Tag {
    title: Option<String>,
    album: Option<String>,
    comment: Option<String>,
    genre: Option<String>,
    year: Option<String>,
}

#[derive(Clone)]
pub struct TagIndex {
    index: Vec<PathBuf>,
    tags: HashMap<PathBuf, Tag>,
}

impl TagIndex {
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
            index: Vec::new(),
        }
    }

    pub fn insert(mut self, p: PathBuf, t: Tag) {
        self.tags.insert(p, t);
    }

    pub fn get(self, p: PathBuf) -> Option<Tag> {
        match self.tags.get(&p) {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }

    pub fn add_from_path(&mut self, path: PathBuf) {
        let tag = Tag::read_from_path(&path).unwrap();
        let e = path.clone();
        &self.index.push(e);
        // println!("added {:?} desu", tag.title());
        &self.tags.insert(path, tag);
    }

    pub fn get_index(&self) -> &Vec<PathBuf> {
        &self.index
    }
}
