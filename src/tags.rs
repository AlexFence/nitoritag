use std::collections::HashMap;
use std::path::PathBuf;
use id3;
use taglib;
use metaflac;

#[derive(Clone)]
pub enum Format {
    MP3,
    OGG,
    FLAC,
}

impl Format {
    pub fn get_format(path: &PathBuf) -> Option<Self> {
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
    format: Format,
    title: Option<String>,
    album: Option<String>,
    artist: Option<String>,
    album_artist: Option<String>,
    year: Option<i32>,
}

// TODO implement new
impl Tag {
    pub fn new(p: &PathBuf) -> Option<Tag> {
        let format = Format::get_format(p).unwrap();

        match format {
            Format::MP3 => Self::create_from_mp3(p, Format::MP3),
            Format::OGG => Self::create_from_ogg(p, Format::OGG),
            Format::FLAC => Self::create_from_flac(p, Format::FLAC),
        }
    }

    fn create_from_mp3(p: &PathBuf, format: Format) -> Option<Tag> {
        let id3tag = id3::Tag::read_from_path(p);

        fn convert_option_to_string(o: Option<&str>) -> Option<String> {
            match o {
                Some(s) => Some(s.to_string()),
                None => None,
            }
        }

        if let Ok(id3tag) = id3tag {
            let title = convert_option_to_string(id3tag.title());
            let album =  convert_option_to_string(id3tag.album());
            let artist = convert_option_to_string(id3tag.artist());
            let album_artist = convert_option_to_string(id3tag.album_artist());
            let year = id3tag.year();


            return Some(Tag{
                format,
                title,
                album,
                artist,
                album_artist,
                year,
            });
        }

        None
    }

    // TODO fix this, it explodes ;w;
    //      stacktrace says it explodes on the unwrap in tagindex.add_from_path
    fn create_from_ogg(p: &PathBuf, f: Format) -> Option<Tag> {
       Self::create_from_taglib(p, f)
    }

    fn create_from_flac(p: &PathBuf, format: Format) -> Option<Tag> {
        // TODO improve this
        // TODO check why the vorbis comments return vecs
        fn convert_vec(vec: Option<&Vec<String>>) -> Option<String> {
            match vec {
                Some(v) => {
                    match &v.clone().pop() {
                        &Some(ref v) => Some(v.clone()),
                        &None => None
                    }
                },
                None => None
            }
        }

        let flac_tag = metaflac::Tag::read_from_path(p);

        if let Ok(flac_tag) = flac_tag {
            let vorbiscomments = match flac_tag.vorbis_comments() {
                Some(s) => s.clone(),
                None => metaflac::block::VorbisComment::new(),
            };

            let title = convert_vec(vorbiscomments.title());
            let album = convert_vec(vorbiscomments.album());
            let artist = convert_vec(vorbiscomments.artist());
            let album_artist = convert_vec(vorbiscomments.album_artist());
            // TODO get the year somehow
            let year = None;

            return Some(Tag{
                format,
                title,
                album,
                artist,
                album_artist,
                year,
            });
        }
        None
    }

    fn create_from_taglib(p: &PathBuf, format: Format) -> Option<Tag> {
    	let file = taglib::File::new(p.to_str().unwrap());

        fn convert_to_option(s: String) -> Option<String> {
            if s != "" {
                return Some(s);
            }

            None
        }

        if let Ok(file) = file {
            let taglib_tag = file.tag().unwrap();
            let title = convert_to_option(taglib_tag.title());
            let album = convert_to_option(taglib_tag.album());
            let artist = convert_to_option(taglib_tag.artist());
            // TODO refactor this (mut)
            let year_temp = taglib_tag.year();
            let mut year;

            if year_temp > 0 {
                year = Some(year_temp as i32);
            } else {
                year = None;
            }

            return Some(Tag{
                format,
                title,
                album,
                album_artist: artist.clone(),
                artist,
                year,
            });
        }

        None
    }

    pub fn title(self) -> Option<String> {
        self.title
    }

    pub fn album(self) -> Option<String> {
        self.album
    }

    pub fn album_artist(self) -> Option<String> {
        self.album_artist
    }

    pub fn artist(self) -> Option<String> {
        self.artist
    }
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
        // TODO fix this
        //      maybe return the success as a boolean?
        //      result would be cleaner
        //      custom errors?
        let tag = Tag::new(&path).unwrap();
        let e = path.clone();
        &self.index.push(e);
        // println!("added {:?} desu", tag.title());
        &self.tags.insert(path, tag);
    }

    pub fn get_index(&self) -> &Vec<PathBuf> {
        &self.index
    }
}
