// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{PageSetup, PrintContext, PrintOperationPreview};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait PrintOperationPreviewImpl: ObjectImpl {
    fn ready(&self, print_operation_preview: &Self::Type, context: &PrintContext) {
        unsafe {
            let type_ = ffi::gtk_print_operation_preview_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkPrintOperationPreviewIface;
            assert!(!iface.is_null());

            ((*iface).ready.as_ref().unwrap())(
                print_operation_preview
                    .unsafe_cast_ref::<PrintOperationPreview>()
                    .to_glib_none()
                    .0,
                context.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn got_page_size(
        &self,
        print_operation_preview: &Self::Type,
        context: &PrintContext,
        page_setup: &PageSetup,
    ) {
        unsafe {
            let type_ = ffi::gtk_print_operation_preview_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkPrintOperationPreviewIface;
            assert!(!iface.is_null());

            ((*iface).got_page_size.as_ref().unwrap())(
                print_operation_preview
                    .unsafe_cast_ref::<PrintOperationPreview>()
                    .to_glib_none()
                    .0,
                context.to_glib_none().0,
                page_setup.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn render_page(&self, print_operation_preview: &Self::Type, page_nr: i32);
    fn is_selected(&self, print_operation_preview: &Self::Type, page_nr: i32) -> bool;
    fn end_preview(&self, print_operation_preview: &Self::Type);
}

unsafe impl<T: PrintOperationPreviewImpl> IsImplementable<T> for PrintOperationPreview {
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let print_operation_preview_iface =
            &mut *(iface as *mut ffi::GtkPrintOperationPreviewIface);

        print_operation_preview_iface.ready = Some(print_operation_preview_ready::<T>);
        print_operation_preview_iface.got_page_size =
            Some(print_operation_preview_got_page_size::<T>);
        print_operation_preview_iface.render_page = Some(print_operation_preview_render_page::<T>);
        print_operation_preview_iface.is_selected = Some(print_operation_preview_is_selected::<T>);
        print_operation_preview_iface.end_preview = Some(print_operation_preview_end_preview::<T>);
    }
}

unsafe extern "C" fn print_operation_preview_ready<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
    contextptr: *mut ffi::GtkPrintContext,
) {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.get_impl();
    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);

    imp.ready(
        from_glib_borrow::<_, PrintOperationPreview>(print_operation_preview).unsafe_cast_ref(),
        &context,
    )
}

unsafe extern "C" fn print_operation_preview_got_page_size<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
    contextptr: *mut ffi::GtkPrintContext,
    setupptr: *mut ffi::GtkPageSetup,
) {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.get_impl();

    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);
    let setup: Borrowed<PageSetup> = from_glib_borrow(setupptr);

    imp.got_page_size(
        from_glib_borrow::<_, PrintOperationPreview>(print_operation_preview).unsafe_cast_ref(),
        &context,
        &setup,
    )
}

unsafe extern "C" fn print_operation_preview_render_page<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
    page_nr: i32,
) {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.get_impl();

    imp.render_page(
        from_glib_borrow::<_, PrintOperationPreview>(print_operation_preview).unsafe_cast_ref(),
        page_nr,
    )
}

unsafe extern "C" fn print_operation_preview_is_selected<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
    page_nr: i32,
) -> glib::ffi::gboolean {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.get_impl();

    imp.is_selected(
        from_glib_borrow::<_, PrintOperationPreview>(print_operation_preview).unsafe_cast_ref(),
        page_nr,
    )
    .to_glib()
}

unsafe extern "C" fn print_operation_preview_end_preview<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
) {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.get_impl();

    imp.end_preview(
        from_glib_borrow::<_, PrintOperationPreview>(print_operation_preview).unsafe_cast_ref(),
    )
}
