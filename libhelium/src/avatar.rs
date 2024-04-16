use crate::Avatar;
use glib::translate::*;
use std::ptr;

impl Avatar {
    #[doc(alias = "he_avatar_new")]
    pub fn new(size: i32, image: Option<&str>, text: Option<&str>, status: Option<bool>) -> Avatar {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::he_avatar_new(
                size,
                image.to_glib_none().0,
                text.to_glib_none().0,
                // this function takes in a nullable boolean, which doesn't have a provided implementation in gtk-rs
                // from what I can see, this is represented as a pointer to a gboolean
                // this should be correct? I hope?
                status
                    .map(|c| &mut c.into_glib() as *mut _)
                    .unwrap_or(ptr::null_mut()),
            ))
        }
    }
}
