use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use id3::Tag;


#[derive(Clone)]
pub struct TagManager {
    index: Vec<PathBuf>,
    tags: HashMap<PathBuf, Tag>,
}

impl TagManager {
    pub fn new() -> Self {
        TagManager {
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
        println!("added {:?} desu", tag.title());
        &self.tags.insert(path, tag);
    }

    pub fn get_index(&self) -> &Vec<PathBuf> {
        &self.index
    }
}
