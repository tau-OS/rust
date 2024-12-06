use crate::{Avatar, AvatarStatusColor};
use glib::translate::*;
use std::ptr;

impl Avatar {
    #[doc(alias = "he_avatar_new")]
    pub fn new(
        size: i32,
        image: Option<&str>,
        text: Option<&str>,
        status: bool,
        status_color: Option<AvatarStatusColor>,
    ) -> Avatar {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::he_avatar_new(
                size,
                image.to_glib_none().0,
                text.to_glib_none().0,
                status.into_glib(),
                status_color
                    .map(|c| &mut c.into_glib() as *mut _)
                    .unwrap_or(ptr::null_mut()),
            ))
        }
    }
}
