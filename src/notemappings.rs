use enigo::Key;
use midi::MidiNote;
use std::path::Path;
use std::io::{BufRead, BufReader, Result};
use std::fs::File;

/// Proxy for Enigo::Key, since that variant isn't cloneable
#[derive(Clone)]
pub enum KbdKey {
    /// return key
    Return,
    /// tab key (tabulator)
    Tab,
    /// space key
    Space,
    /// backspace key
    Backspace,
    /// escape key (esc)
    Escape,
    /// super key on linux (command key on macOS, windows key on Windows)
    Super,
    /// command key on macOS (super key on Linux, windows key on Windows)
    Command,
    /// windows key on Windows (super key on Linux, command key on macOS)
    Windows,
    /// shift key
    Shift,
    /// caps lock key
    CapsLock,
    /// alt key on Linux and Windows (option key on macOS)
    Alt,
    /// option key on macOS (alt key on Linux and Windows)
    Option,
    /// control key
    Control,
    /// home key
    Home,
    /// page up key
    PageUp,
    /// page down key
    PageDown,
    /// left arrow key
    LeftArrow,
    /// right arrow key
    RightArrow,
    /// down arrow key
    DownArrow,
    /// up arrow key
    UpArrow,
    /// F1 key
    F1,
    /// F2 key
    F2,
    /// F3 key
    F3,
    /// F4 key
    F4,
    /// F5 key
    F5,
    /// F6 key
    F6,
    /// F7 key
    F7,
    /// F8 key
    F8,
    /// F9 key
    F9,
    /// F10 key
    F10,
    /// F11 key
    F11,
    /// F12 key
    F12,
    /// keyboard layout dependent key
    Layout(char),
    /// raw keycode eg 0x38
    Raw(u16),
}

impl KbdKey {
    pub fn to_enigo_key(obj: &KbdKey) -> Key {
        match *obj {
            KbdKey::Return => Key::Return,
            KbdKey::Tab => Key::Tab,
            KbdKey::Space => Key::Space,
            KbdKey::Backspace => Key::Backspace,
            KbdKey::Escape => Key::Escape,
            KbdKey::Super => Key::Super,
            KbdKey::Command => Key::Command,
            KbdKey::Windows => Key::Windows,
            KbdKey::Shift => Key::Shift,
            KbdKey::CapsLock => Key::CapsLock,
            KbdKey::Alt => Key::Alt,
            KbdKey::Option => Key::Option,
            KbdKey::Control => Key::Control,
            KbdKey::Home => Key::Home,
            KbdKey::PageUp => Key::PageUp,
            KbdKey::PageDown => Key::PageDown,
            KbdKey::LeftArrow => Key::LeftArrow,
            KbdKey::RightArrow => Key::RightArrow,
            KbdKey::DownArrow => Key::DownArrow,
            KbdKey::UpArrow => Key::UpArrow,
            KbdKey::F1 => Key::F1,
            KbdKey::F2 => Key::F2,
            KbdKey::F3 => Key::F3,
            KbdKey::F4 => Key::F4,
            KbdKey::F5 => Key::F5,
            KbdKey::F6 => Key::F6,
            KbdKey::F7 => Key::F7,
            KbdKey::F8 => Key::F8,
            KbdKey::F9 => Key::F9,
            KbdKey::F10 => Key::F10,
            KbdKey::F11 => Key::F11,
            KbdKey::F12 => Key::F12,
            KbdKey::Layout(c) => Key::Layout(c),
            KbdKey::Raw(c) => Key::Raw(c),
        }
    }
}

#[derive(Clone)]
pub enum Event {
    Delay(u64),
    KeyDown(KbdKey),
    KeyUp(KbdKey),
}

#[derive(Clone)]
pub struct NoteMapping {
    /// The source note that triggered this event.
    note: MidiNote,

    /// The source channel.  0 is a good default here.
    channel: u8,

    pub on: Vec<Event>,
    pub off: Vec<Event>,
}

pub struct NoteMappings {
    mappings: Vec<NoteMapping>,
}

impl NoteMappings {
    pub fn new() -> NoteMappings {
        NoteMappings { mappings: vec![] }
    }

    /// Find a mapping for a given note, if one exists
    pub fn find(&self, note: &MidiNote, channel: u8) -> Option<NoteMapping> {
        for mapping in &self.mappings {
            if mapping.note == *note && mapping.channel == channel {
                return Some(mapping.clone());
            }
        }
        None
    }

    pub fn import(&mut self, filename: &str) -> Result<()> {
        let f = File::open(filename)?;
        let mut buf_reader = BufReader::new(f);
        for line in buf_reader.lines() {
            let l = line.unwrap();
            let fields: Vec<&str> = l.split(" ").collect();
            if fields.len() != 4 {
                println!("Line is not 4 elements!");
                continue;
            }
            let note_txt = fields[0];
            let channel_txt = fields[1];
            let keydown_txt = fields[2];
            let keyup_txt = fields[3];
            let note = MidiNote::new_from_text(&note_txt);
            println!("Got line: {}  Note: {:?}", l, note);
        }
        Ok(())
    }
}
