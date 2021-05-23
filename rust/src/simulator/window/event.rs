use std::ops::{BitAnd, BitOr, BitXor};
use std::collections::HashMap;

pub struct EventManager {
    events: Vec<Event>,
    keymap: HashMap<usize, u16>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            keymap: HashMap::new()
        }
    }
    
    pub fn push_event<T>(&mut self, e: T) where T: Into<Event> {
        self.events.push(e.into());
    }

    pub fn register_key_down(&mut self, key: usize) {
            self.keymap.entry(key)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            let count = self.keymap[&key];
            if count == 1 {
                self.push_event(KeyboardEvent::KEYDOWN(key));
            }
            else {
                self.push_event(KeyboardEvent::KEYHELD(key, count));
            }
    }

    pub fn register_key_up(&mut self, key: usize) {
        if self.keymap.contains_key(&key) {
            let count = self.keymap[&key];
            self.push_event(KeyboardEvent::KEYPRESS(key, count));
            self.keymap.insert(key, 0);
        }
        self.push_event(KeyboardEvent::KEYUP(key));
    }

    pub fn peek_iter(&mut self) -> EventIterator {
        EventIterator {
            data: &mut self.events,
            consuming: false,
            filter: EventFilter::NOFILTER as usize
        }
    }
    
    pub fn poll_iter(&mut self) -> EventIterator {
        EventIterator {
            data: &mut self.events,
            consuming: true,
            filter: EventFilter::NOFILTER as usize
        }
    }

    pub fn flush(&mut self) -> usize {
        let number_of_events = self.events.len();
        self.events = Vec::new();
        number_of_events
    }
}

pub struct EventIterator<'a> {
    data: &'a mut Vec<Event>,
    consuming: bool,
    filter: usize
}

// add filter method
impl<'a> EventIterator<'a> {
    pub fn peek_last(&mut self) -> Option<Event> {
        if self.data.len() == 0 {
            None
        }
        else {
            Some(self.data[self.data.len() - 1])
        }
    }

    pub fn filter<T>(mut self, filter: T) -> Self where T: Into<usize> {
        self.ref_filter(filter);
        self
    }

    pub fn ref_filter<T>(&mut self, filter: T) -> &mut Self where T: Into<usize> {
        self.filter = filter.into();
        self
    }

    pub fn add_filter(mut self, filter_to_add: EventFilter) -> Self {
        self.ref_add_filter(filter_to_add);
        self
    }

    pub fn ref_add_filter(&mut self, filter_to_add: EventFilter) -> &mut Self {
        self.filter |= filter_to_add as usize;
        self
    }

    pub fn remove_filter(&mut self, filter_to_remove: EventFilter) -> &mut Self {
        self.ref_remove_filter(filter_to_remove);
        self
    }

    pub fn ref_remove_filter(&mut self, filter_to_remove: EventFilter) -> &mut Self {
        self.filter ^= filter_to_remove as usize;
        self
    }
}

impl<'a> Iterator for EventIterator<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.data.len() {
            if self.data[i].get_filter_type() as usize & self.filter != 0 {
                if self.consuming {
                    return self.data.pop()
                }
                else {
                    return self.peek_last()
                }
            }
        }
        None
    }
}

#[derive(Copy, Clone, Debug)]
pub enum EventFilter {
    WINDOWEVENT = 0b001,
    KEYBOARDEVENT = 0b010,
    MOUSEEVENT = 0b100,
    NOFILTER = 0b111
}

impl Into<usize> for EventFilter {
    fn into(self) -> usize {
        self as usize
    }
}

impl BitAnd for EventFilter {
    type Output = usize;
    fn bitand(self, rhs: Self) -> Self::Output {
        self as Self::Output & rhs as Self::Output
    }
}

impl BitAnd<usize> for EventFilter {
    type Output = usize;
    fn bitand(self, rhs: usize) -> Self::Output {
        self as usize & rhs
    }
}

impl BitOr for EventFilter {
    type Output = usize;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as Self::Output | rhs as Self::Output
    }
}

impl BitOr<usize> for EventFilter {
    type Output = usize;
    fn bitor(self, rhs: usize) -> Self::Output {
        self as usize | rhs
    }
}

impl BitXor for EventFilter {
    type Output = usize;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self as Self::Output ^ rhs as Self::Output
    }
}

impl BitXor<usize> for EventFilter {
    type Output = usize;
    fn bitxor(self, rhs: usize) -> Self::Output {
        self as usize ^ rhs
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    KEYDOWN(KeyboardEvent),
    KEYUP(KeyboardEvent),
    KEYHELD(KeyboardEvent),
    KEYPRESS(KeyboardEvent),

    MOUSEDOWN(MouseEvent),
    MOUSEUP(MouseEvent),
    MOUSESCROLL(MouseEvent),
    MOUSEMOVE,

    WINDOWRESIZE(WindowEvent),
    WINDOWMOVE(WindowEvent), // need impl
    WINDOWFOCUS(WindowEvent),
    WINDOWBLUR(WindowEvent),
    WINDOWMAXIMIZE(WindowEvent),
    WINDOWUNMAXIMIZE(WindowEvent),
    WINDOWMINIMIZE(WindowEvent),
    WINDOWRESTORE(WindowEvent),
    WINDOWCLOSEBEGIN(WindowEvent),
    WINDOWCLOSEFINAL(WindowEvent),

    UNKNOWN(()),
}

impl Event {
    pub fn get_filter_type(&self) -> EventFilter {
        match self {
            Self::KEYDOWN(_) | Self::KEYUP(_) | Self::KEYHELD(_) |Self::KEYPRESS(_) => EventFilter::KEYBOARDEVENT,

            Self::MOUSEDOWN(_) | Self::MOUSEUP(_) | Self::MOUSESCROLL(_) |Self::MOUSEMOVE => EventFilter::MOUSEEVENT,

            Self::WINDOWRESIZE(_) | Self::WINDOWMOVE(_) | Self::WINDOWFOCUS(_) | Self::WINDOWBLUR(_) | Self::WINDOWMAXIMIZE(_) | Self::WINDOWUNMAXIMIZE(_) |
                Self::WINDOWMINIMIZE(_) | Self::WINDOWRESTORE(_) | Self::WINDOWCLOSEBEGIN(_) | Self::WINDOWCLOSEFINAL(_) => EventFilter::WINDOWEVENT,

            _ => EventFilter::NOFILTER
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum KeyboardEvent {
    KEYDOWN(usize), // keycode
    KEYUP(usize), // keycode
    // called when a key is held, keypress called when a held key is released
    KEYHELD(usize, u16), // keycode, repeated
    KEYPRESS(usize, u16), // keycode, repeated
}

impl Into<Event> for KeyboardEvent {
    fn into(self) -> Event {
        #[allow(unreachable_patterns)]
        match self {
            Self::KEYDOWN(_) => Event::KEYDOWN(self),
            Self::KEYUP(_) => Event::KEYUP(self),
            Self::KEYHELD(_, _) => Event::KEYHELD(self),
            Self::KEYPRESS(_, _) => Event::KEYPRESS(self),
            _ => Event::UNKNOWN(())
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MouseButton {
    LEFTMOUSE = 1,
    RIGHTMOUSE = 2,
    MIDDLEMOUSE = 3,
    XBUTTON = 4,
    YBUTTON = 5,
}

impl Into<u8> for MouseButton {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MouseEvent {
    MOUSEDOWN(MouseButton, i32, i32), // button, x_pos, y_pos
    MOUSEUP(MouseButton, i32, i32), // button, x_pos, y_pos
    // called when a mouse button is held, mousepress called when a held mouse button is released
    MOUSESCROLL(i16, i16, i32, i32), // x_direction, y_direction, x_pos, y_pos
    MOUSEMOVE
}

impl Into<Event> for MouseEvent {
    fn into(self) -> Event {
        #[allow(unreachable_patterns)]
        match self {
            Self::MOUSEDOWN(_, _, _) => Event::MOUSEDOWN(self),
            Self::MOUSEUP(_, _, _) => Event::MOUSEUP(self),
            Self::MOUSESCROLL(_, _, _, _) => Event::MOUSESCROLL(self),
            Self::MOUSEMOVE => Event::MOUSEMOVE,
            _ => Event::UNKNOWN(())
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum WindowEvent {
    WINDOWRESIZE,
    WINDOWMOVE,
    WINDOWFOCUS,
    WINDOWBLUR,
    WINDOWMAXIMIZE,
    WINDOWUNMAXIMIZE,
    WINDOWMINIMIZE,
    WINDOWRESTORE,
    WINDOWCLOSEBEGIN, // called at the beginning of closing the window; allows for cancelling the close
    WINDOWCLOSEFINAL, // called at the end of closing the window; cannot be reversed
}

impl Into<Event> for WindowEvent {
    fn into(self) -> Event {
        #[allow(unreachable_patterns)]
        match self {
            Self::WINDOWRESIZE => Event::WINDOWRESIZE(self),
            Self::WINDOWMOVE => Event::WINDOWMOVE(self),
            Self::WINDOWFOCUS => Event::WINDOWFOCUS(self),
            Self::WINDOWBLUR => Event::WINDOWBLUR(self),
            Self::WINDOWMAXIMIZE => Event::WINDOWMAXIMIZE(self),
            Self::WINDOWUNMAXIMIZE => Event::WINDOWUNMAXIMIZE(self),
            Self::WINDOWMINIMIZE => Event::WINDOWMINIMIZE(self),
            Self::WINDOWRESTORE => Event::WINDOWRESTORE(self),
            Self::WINDOWCLOSEBEGIN => Event::WINDOWCLOSEBEGIN(self),
            Self::WINDOWCLOSEFINAL => Event::WINDOWCLOSEFINAL(self),
            _ => Event::UNKNOWN(())
        }
    }
}