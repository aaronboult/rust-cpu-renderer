use std::ops::{BitAnd, BitOr, BitXor};

pub struct EventManager {
    events: Vec<Event>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            events: Vec::new()
        }
    }
    
    pub fn push_event<T>(&mut self, e: T) where T: Into<Event> {
        self.events.push(e.into());
    }

    pub fn peek_iter(&mut self) -> EventIterator {
        EventIterator {
            data: &mut self.events,
            consuming: false,
            filter: EventFilter::NOFILTER as isize
        }
    }
    
    pub fn poll_iter(&mut self) -> EventIterator {
        EventIterator {
            data: &mut self.events,
            consuming: true,
            filter: EventFilter::NOFILTER as isize
        }
    }
}

pub struct EventIterator<'a> {
    data: &'a mut Vec<Event>,
    consuming: bool,
    filter: isize
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

    pub fn filter(&mut self, filter: EventFilter) {
        self.filter = filter as isize;
    }

    pub fn add_filter(&mut self, filter_to_add: EventFilter) {
        self.filter |= filter_to_add as isize;
    }

    pub fn remove_filter(&mut self, filter_to_remove: EventFilter) {
        self.filter ^= filter_to_remove as isize;
    }
}

impl<'a> Iterator for EventIterator<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.data.len() {
            if self.data[i].get_filter_type() as isize & self.filter != 0 {
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

impl BitAnd for EventFilter {
    type Output = isize;
    fn bitand(self, rhs: Self) -> Self::Output {
        self as Self::Output & rhs as Self::Output
    }
}

impl BitAnd<isize> for EventFilter {
    type Output = isize;
    fn bitand(self, rhs: isize) -> Self::Output {
        self as isize & rhs
    }
}

impl BitOr for EventFilter {
    type Output = isize;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as Self::Output | rhs as Self::Output
    }
}

impl BitOr<isize> for EventFilter {
    type Output = isize;
    fn bitor(self, rhs: isize) -> Self::Output {
        self as isize | rhs
    }
}

impl BitXor for EventFilter {
    type Output = isize;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self as Self::Output ^ rhs as Self::Output
    }
}

impl BitXor<isize> for EventFilter {
    type Output = isize;
    fn bitxor(self, rhs: isize) -> Self::Output {
        self as isize ^ rhs
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
    MOUSEHELD(MouseEvent),
    MOUSEPRESS(MouseEvent),
    MOUSESCROLL(MouseEvent),
    MOUSEMOVE,

    WINDOWRESIZE(WindowEvent),
    WINDOWMOVE(WindowEvent),
    WINDOWFOCUS(WindowEvent),
    WINDOWBLUR(WindowEvent),
    WINDOWMAXIMIZE(WindowEvent),
    WINDOWMINIMIZE(WindowEvent),
    WINDOWRESTORE(WindowEvent),
    WINDOWCLOSEBEGIN(WindowEvent),
    WINDOWCLOSEFINAL(WindowEvent),

    UNKNOWN(()),
}

impl Event {
    pub fn get_filter_type(&self) -> EventFilter {
        match self {
            Self::KEYDOWN(_) | Self::KEYUP(_) | Self::KEYHELD(_) | Self::KEYPRESS(_) => EventFilter::KEYBOARDEVENT,
            Self::MOUSEDOWN(_) | Self::MOUSEUP(_) | Self::MOUSEHELD(_) | Self::MOUSEPRESS(_) | Self::MOUSESCROLL(_) => EventFilter::MOUSEEVENT,
            Self::WINDOWRESIZE(_) | Self::WINDOWMOVE(_) | Self::WINDOWFOCUS(_) | Self::WINDOWBLUR(_) | Self::WINDOWMAXIMIZE(_) |
                Self::WINDOWMINIMIZE(_) | Self::WINDOWRESTORE(_) | Self::WINDOWCLOSEBEGIN(_) | Self::WINDOWCLOSEFINAL(_) => EventFilter::WINDOWEVENT,
            _ => EventFilter::NOFILTER
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum KeyboardEvent {
    KEYDOWN(u32), // keycode
    KEYUP(u32), // keycode
    // called when a key is held, keypress called when a held key is released
    KEYHELD(u32, u32), // keycode, repeated
    KEYPRESS(u32, u32), // keycode, repeated
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
pub enum MouseEvent {
    MOUSEDOWN(u8), // button
    MOUSEUP(u8), // button
    // called when a mouse button is held, mousepress called when a held mouse button is released
    MOUSEHELD(u8, u32), // button, repeated
    MOUSEPRESS(u8, u32), // keycode, repeated
    MOUSESCROLL(i8, i8), // x_direction, y_direction
    MOUSEMOVE
}

impl Into<Event> for MouseEvent {
    fn into(self) -> Event {
        #[allow(unreachable_patterns)]
        match self {
            Self::MOUSEDOWN(_) => Event::MOUSEDOWN(self),
            Self::MOUSEUP(_) => Event::MOUSEUP(self),
            Self::MOUSEHELD(_, _) => Event::MOUSEHELD(self),
            Self::MOUSEPRESS(_, _) => Event::MOUSEPRESS(self),
            Self::MOUSESCROLL(_, _) => Event::MOUSESCROLL(self),
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
            Self::WINDOWMINIMIZE => Event::WINDOWMINIMIZE(self),
            Self::WINDOWRESTORE => Event::WINDOWRESTORE(self),
            Self::WINDOWCLOSEBEGIN => Event::WINDOWCLOSEBEGIN(self),
            Self::WINDOWCLOSEFINAL => Event::WINDOWCLOSEFINAL(self),
            _ => Event::UNKNOWN(())
        }
    }
}