use crate::Fuse;
use glib::{subclass::prelude::*, translate::ToGlibPtr};

pub trait FuseImpl: ObjectImpl {
    fn get_widget(&self) -> gtk::Widget;
    fn shown(&self);
    fn hidden(&self);
}

unsafe impl<T: FuseImpl> IsSubclassable<T> for Fuse {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.get_widget = Some(get_widget::<T>);
        klass.shown = Some(shown::<T>);
        klass.hidden = Some(hidden::<T>);
    }
}

unsafe extern "C" fn get_widget<T: FuseImpl>(
    ptr: *mut ffi::FuseboxFuse,
) -> *mut gtk::ffi::GtkWidget {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.get_widget().to_glib_full()
}

unsafe extern "C" fn shown<T: FuseImpl>(ptr: *mut ffi::FuseboxFuse) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.shown();
}

unsafe extern "C" fn hidden<T: FuseImpl>(ptr: *mut ffi::FuseboxFuse) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.hidden();
}
