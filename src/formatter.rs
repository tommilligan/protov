use serde_json::ser::{Formatter, PrettyFormatter};
use std::io::{self, Write};

pub struct CustomFormatter<'a> {
    formatter: PrettyFormatter<'a>,
    depth: usize,
}

impl<'a> CustomFormatter<'a> {
    pub fn new() -> Self {
        CustomFormatter {
            formatter: PrettyFormatter::default(),
            depth: 0,
        }
    }
}

impl<'a> Formatter for &'a mut CustomFormatter<'a> {
    fn begin_array<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.formatter.begin_array(w)
    }
    fn end_array<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.formatter.end_array(w)
    }
    fn begin_array_value<W: ?Sized + Write>(&mut self, w: &mut W, first: bool) -> io::Result<()> {
        self.formatter.begin_array_value(w, first)
    }
    fn end_array_value<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.formatter.end_array_value(w)
    }
    fn begin_object<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.depth += 1;
        self.formatter.begin_object(w)
    }
    fn end_object<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.formatter.end_object(w).and_then(|()| {
            self.depth -= 1;
            if self.depth == 0 {
                w.write_all(b"\n")
            } else {
                Ok(())
            }
        })
    }
    fn begin_object_key<W: ?Sized + Write>(&mut self, w: &mut W, first: bool) -> io::Result<()> {
        self.formatter.begin_object_key(w, first)
    }
    fn begin_object_value<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.formatter.begin_object_value(w)
    }
    fn end_object_value<W: ?Sized + Write>(&mut self, w: &mut W) -> io::Result<()> {
        self.formatter.end_object_value(w)
    }
}
