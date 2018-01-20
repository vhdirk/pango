// This file was generated by gir (c6b70b0) from gir-files (469db10)
// DO NOT EDIT

use Font;
use Rectangle;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct GlyphString(Boxed<ffi::PangoGlyphString>);

    match fn {
        copy => |ptr| ffi::pango_glyph_string_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_glyph_string_free(ptr),
        get_type => || ffi::pango_glyph_string_get_type(),
    }
}

impl GlyphString {
    pub fn new() -> GlyphString {
        unsafe {
            from_glib_full(ffi::pango_glyph_string_new())
        }
    }

    pub fn extents(&mut self, font: &Font) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_glyph_string_extents(self.to_glib_none_mut().0, font.to_glib_none().0, ink_rect.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            (ink_rect, logical_rect)
        }
    }

    pub fn extents_range(&mut self, start: i32, end: i32, font: &Font) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_glyph_string_extents_range(self.to_glib_none_mut().0, start, end, font.to_glib_none().0, ink_rect.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            (ink_rect, logical_rect)
        }
    }

    //pub fn get_logical_widths(&mut self, text: &str, embedding_level: i32, logical_widths: &[i32]) {
    //    unsafe { TODO: call ffi::pango_glyph_string_get_logical_widths() }
    //}

    pub fn get_width(&mut self) -> i32 {
        unsafe {
            ffi::pango_glyph_string_get_width(self.to_glib_none_mut().0)
        }
    }

    //pub fn index_to_x(&mut self, text: &str, analysis: /*Ignored*/&mut Analysis, index_: i32, trailing: bool) -> i32 {
    //    unsafe { TODO: call ffi::pango_glyph_string_index_to_x() }
    //}

    pub fn set_size(&mut self, new_len: i32) {
        unsafe {
            ffi::pango_glyph_string_set_size(self.to_glib_none_mut().0, new_len);
        }
    }

    //pub fn x_to_index(&mut self, text: &str, analysis: /*Ignored*/&mut Analysis, x_pos: i32) -> (i32, i32) {
    //    unsafe { TODO: call ffi::pango_glyph_string_x_to_index() }
    //}
}

impl Default for GlyphString {
    fn default() -> Self {
        Self::new()
    }
}
