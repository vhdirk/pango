// This file was generated by gir (94e079d) from gir-files (469db10)
// DO NOT EDIT

use Context;
use Font;
use FontDescription;
use FontFamily;
use Fontset;
use Language;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FontMap(Object<ffi::PangoFontMap>);

    match fn {
        get_type => || ffi::pango_font_map_get_type(),
    }
}

pub trait FontMapExt {
    #[cfg(any(feature = "v1_34", feature = "dox"))]
    fn changed(&self);

    fn create_context(&self) -> Option<Context>;

    #[cfg(any(feature = "v1_32_4", feature = "dox"))]
    fn get_serial(&self) -> u32;

    fn get_shape_engine_type(&self) -> Option<String>;

    fn list_families(&self) -> Vec<FontFamily>;

    fn load_font(&self, context: &Context, desc: &FontDescription) -> Option<Font>;

    fn load_fontset(&self, context: &Context, desc: &FontDescription, language: &Language) -> Option<Fontset>;
}

impl<O: IsA<FontMap>> FontMapExt for O {
    #[cfg(any(feature = "v1_34", feature = "dox"))]
    fn changed(&self) {
        unsafe {
            ffi::pango_font_map_changed(self.to_glib_none().0);
        }
    }

    fn create_context(&self) -> Option<Context> {
        unsafe {
            from_glib_full(ffi::pango_font_map_create_context(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_32_4", feature = "dox"))]
    fn get_serial(&self) -> u32 {
        unsafe {
            ffi::pango_font_map_get_serial(self.to_glib_none().0)
        }
    }

    fn get_shape_engine_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::pango_font_map_get_shape_engine_type(self.to_glib_none().0))
        }
    }

    fn list_families(&self) -> Vec<FontFamily> {
        unsafe {
            let mut families = ptr::null_mut();
            let mut n_families = mem::uninitialized();
            ffi::pango_font_map_list_families(self.to_glib_none().0, &mut families, &mut n_families);
            FromGlibContainer::from_glib_container_num(families, n_families as usize)
        }
    }

    fn load_font(&self, context: &Context, desc: &FontDescription) -> Option<Font> {
        unsafe {
            from_glib_full(ffi::pango_font_map_load_font(self.to_glib_none().0, context.to_glib_none().0, desc.to_glib_none().0))
        }
    }

    fn load_fontset(&self, context: &Context, desc: &FontDescription, language: &Language) -> Option<Fontset> {
        unsafe {
            from_glib_full(ffi::pango_font_map_load_fontset(self.to_glib_none().0, context.to_glib_none().0, desc.to_glib_none().0, mut_override(language.to_glib_none().0)))
        }
    }
}
