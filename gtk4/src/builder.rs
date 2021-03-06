// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Builder;
use glib::object::{Cast, IsA};
use glib::translate::*;
use glib::Object;
use std::path::Path;

impl Builder {
    #[doc(alias = "gtk_builder_new_from_file")]
    pub fn new_from_file<T: AsRef<Path>>(file_path: T) -> Builder {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_builder_new_from_file(
                file_path.as_ref().to_glib_none().0,
            ))
        }
    }
}

pub trait BuilderExtManual: 'static {
    #[doc(alias = "gtk_builder_get_object")]
    fn get_object<T: IsA<Object>>(&self, name: &str) -> Option<T>;

    #[doc(alias = "gtk_builder_add_from_file")]
    fn add_from_file<T: AsRef<Path>>(&self, file_path: T) -> Result<(), glib::Error>;
}

impl<O: IsA<Builder>> BuilderExtManual for O {
    fn get_object<T: IsA<Object>>(&self, name: &str) -> Option<T> {
        unsafe {
            Option::<Object>::from_glib_none(ffi::gtk_builder_get_object(
                self.upcast_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
            .and_then(|obj| obj.dynamic_cast::<T>().ok())
        }
    }

    fn add_from_file<T: AsRef<Path>>(&self, file_path: T) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ::std::ptr::null_mut();
            ffi::gtk_builder_add_from_file(
                self.upcast_ref().to_glib_none().0,
                file_path.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
