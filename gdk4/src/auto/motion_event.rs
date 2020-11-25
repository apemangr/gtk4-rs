// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Event;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct MotionEvent(Object<ffi::GdkMotionEvent>) @extends Event;

    match fn {
        get_type => || ffi::gdk_motion_event_get_type(),
    }
}

impl MotionEvent {}

impl fmt::Display for MotionEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MotionEvent")
    }
}