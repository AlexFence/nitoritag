use id3;
use metaflac;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use taglib;

#[derive(Copy, Clone, Debug)]
pub enum Format {
    MP3,
    OGG,
    FLAC,
    // For dynamically created Tag objects
    // that were not read from a file
    DYNAMIC,
}

impl Format {
    pub fn get_format(path: &PathBuf) -> Option<Self> {
        match path.extension() {
            Some(ext) => match ext.to_str().unwrap() {
                "mp3" => Some(Format::MP3),
                "ogg" => Some(Format::OGG),
                "flac" => Some(Format::FLAC),
                _ => None,
            },
            None => None,
        }
    }

    pub fn file_is_supported(path: &PathBuf) -> bool {
        return Self::get_format(path).is_some();
    }
}

#[derive(Clone, Debug)]
pub struct Tag {
    format: Format,
    title: Option<String>,
    album: Option<String>,
    artist: Option<String>,
    album_artist: Option<String>,
    year: Option<i32>,
}

impl Tag {
    pub fn new(
        title: Option<String>,
        album: Option<String>,
        artist: Option<String>,
        album_artist: Option<String>,
    ) -> Tag {
        Tag {
            format: Format::DYNAMIC,
            year: None,
            title,
            album,
            artist,
            album_artist,
        }
    }

    pub fn from_path(p: &PathBuf) -> Option<Tag> {
        let format = Format::get_format(p).unwrap();

        match format {
            Format::MP3 => Self::create_from_mp3(p, Format::MP3),
            Format::OGG => Self::create_from_ogg(p, Format::OGG),
            Format::FLAC => Self::create_from_flac(p, Format::FLAC),
            _ => None,
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
            let album = convert_option_to_string(id3tag.album());
            let artist = convert_option_to_string(id3tag.artist());
            let album_artist = convert_option_to_string(id3tag.album_artist());
            let year = id3tag.year();

            return Some(Tag {
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
                Some(v) => match &v.clone().pop() {
                    &Some(ref v) => Some(v.clone()),
                    &None => None,
                },
                None => None,
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

            return Some(Tag {
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

        if let Ok(file) = file {
            let taglib_tag = file.tag().unwrap();
            let title = taglib_tag.title();
            let album = taglib_tag.album();
            let artist = taglib_tag.artist();
            let year_unsigned = taglib_tag.year();

            let mut year: Option<i32> = None;

            if year_unsigned.is_some() {
                year = Some(year_unsigned.unwrap() as i32);
            }

            return Some(Tag {
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

    pub fn get(self, p: &PathBuf) -> Option<Tag> {
        match self.tags.get(p) {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }

    pub fn add_from_path(&mut self, path: PathBuf) -> io::Result<()> {
        if path.is_dir() {
            for entry in fs::read_dir(&path)? {
                let entry = entry?;
                let path = entry.path();
                self.add_from_path(path)?;
            }
        } else {
            self.add_file_path(path)
        }
        Ok(())
    }

    pub fn get_index(&self) -> &Vec<PathBuf> {
        &self.index
    }

    fn add_file_path(&mut self, path: PathBuf) {
        if Format::file_is_supported(&path) {
            let tag = Tag::from_path(&path).unwrap();
            let e = path.clone();
            &self.index.push(e);
            &self.tags.insert(path, tag);
        } else {
            eprintln!(
                "{} is not a supported audio file!",
                path.into_os_string().into_string().unwrap()
            );
        }
    }
}

fn write_id3(new_values: &Tag, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("new values: {:?}", new_values);
    let mut id3_tag = id3::Tag::read_from_path(path)?;

    let title_option = new_values.clone().title();
    let artist_option = new_values.clone().artist();
    let album_option = new_values.clone().album();
    let album_artist_option = new_values.clone().album_artist();

    if let Some(title) = title_option {
        id3_tag.set_title(title);
    }

    if let Some(artist) = artist_option {
        id3_tag.set_artist(artist);
    }

    if let Some(album) = album_option {
        id3_tag.set_album(album);
    }

    if let Some(album_artist) = album_artist_option {
        id3_tag.set_album_artist(album_artist);
    }

    id3_tag.write_to_path(path, id3::Version::Id3v24)?;
    Ok(())
}

pub fn write_tag(new_values: &Tag, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    match Format::get_format(path).unwrap() {
        Format::MP3 => write_id3(new_values, path),
        _ => {
            eprintln!("format write unimplemented!");
            Ok(())
        }
    }
}

pub fn write_tags(new_values: &Tag, files: &Vec<PathBuf>) {
    files.iter().for_each(|path| {
        match write_tag(new_values, path) {
            Ok(_) => println!("wrote tags for {:?}", path),
            Err(_) => eprintln!("failed to write tags for {:?}", path),
        };
    })
}
