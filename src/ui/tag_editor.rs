use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use gtk::prelude::{BuilderExtManual, ComboBoxExtManual};
use gtk::{Builder, CellLayoutExt, ComboBoxTextExt, EntryExt};

use tags::Tag;
use ui::action_bus::ActionBus;
use ui::{Component, EditorComponent};

pub struct TagEditor {
    root: gtk::Box,
    title: gtk::ComboBoxText,
    artist: gtk::ComboBoxText,
    album_artist: gtk::ComboBoxText,
    album: gtk::ComboBoxText,
    tags: Vec<Tag>,
    action_bus: Rc<RefCell<ActionBus>>,
}

impl TagEditor {
    pub fn new(action_bus: Rc<RefCell<ActionBus>>) -> Self {
        let window_src = include_str!("tag_editor.glade");
        let builder = Builder::new_from_string(window_src);

        let root: gtk::Box = builder.get_object("tag_editor").unwrap();
        let title: gtk::ComboBoxText = builder.get_object("title_entry").unwrap();
        let artist: gtk::ComboBoxText = builder.get_object("artist_entry").unwrap();
        let album_artist: gtk::ComboBoxText = builder.get_object("album_artist_entry").unwrap();
        let album: gtk::ComboBoxText = builder.get_object("album_entry").unwrap();

        let tags: Vec<Tag> = Vec::new();
        Self {
            root,
            title,
            artist,
            album_artist,
            album,
            tags,
            action_bus,
        }
    }

    fn clear_fields(&mut self) {
        self.title.remove_all();
        self.artist.remove_all();
        self.album.remove_all();
        self.album_artist.remove_all();
    }

    fn populate_entry(entry: &gtk::ComboBoxText, values: &HashSet<String>) {
        values.iter().for_each(|x| entry.append_text(x.as_str()));

        if values.len() > 1 {
            entry.prepend_text("<keep>");
        }

        entry.set_active(Some(0));
    }

    fn update_fields(&mut self) {
        self.clear_fields();

        let mut unique_titles: HashSet<String> = HashSet::new();
        let mut unique_artists: HashSet<String> = HashSet::new();
        let mut unique_album_artists: HashSet<String> = HashSet::new();
        let mut unique_albums: HashSet<String> = HashSet::new();

        for tag in self.tags.clone() {
            let title_option = tag.clone().title();
            let artist_option = tag.clone().artist();
            let album_option = tag.clone().album();
            let album_artist_option = tag.clone().album_artist();

            // we have to handle None values as empty strings for the ui to make sense
            // no value is also a value for the combo boxes
            match title_option {
                Some(title) => unique_titles.insert(title),
                None => unique_titles.insert(String::new()),
            };

            match artist_option {
                Some(artists) => unique_artists.insert(artists),
                None => unique_artists.insert(String::new()),
            };

            match album_option {
                Some(album) => unique_albums.insert(album),
                None => unique_albums.insert(String::new()),
            };

            match album_artist_option {
                Some(album_artist) => unique_album_artists.insert(album_artist),
                None => unique_album_artists.insert(String::new()),
            };
        }

        Self::populate_entry(&self.title, &unique_titles);
        Self::populate_entry(&self.artist, &unique_artists);
        Self::populate_entry(&self.album, &unique_albums);
        Self::populate_entry(&self.album_artist, &unique_album_artists);
    }
}

impl Component<gtk::Box> for TagEditor {
    fn get_root_widget(&mut self) -> &mut gtk::Box {
        &mut self.root
    }
}

impl EditorComponent for TagEditor {
    fn set_tags_to_edit(&mut self, tags: Vec<Tag>) {
        println!("Received tags:");
        for t in tags.clone() {
            println!("{:?}", t);
        }
        self.tags = tags;
        self.update_fields();
    }

    fn get_new_values(&self) -> Tag {
        let title = self.title.get_active_text().unwrap().to_string();
        let artist = self.title.get_active_text().unwrap().to_string();
        let album = self.title.get_active_text().unwrap().to_string();
        let album_artist = self.title.get_active_text().unwrap().to_string();

        let mut title_opt = None;
        let mut artist_opt = None;
        let mut album_opt = None;
        let mut album_artist_opt = None;

        if !title.eq("<keep>") {
            title_opt = Some(title);
        }

        if !album.eq("<keep>") {
            album_opt = Some(album);
        }

        if !artist.eq("<keep>") {
            artist_opt = Some(artist);
        }

        if !album_artist.eq("<keep>") {
            album_artist_opt = Some(album_artist);
        }

        Tag::new(title_opt, album_opt, artist_opt, album_artist_opt)
    }
}
