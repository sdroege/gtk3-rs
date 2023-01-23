// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{CellArea, CellRenderer, TreeIter, TreeModel};
use glib::{prelude::*, translate::*};
use std::{boxed::Box as Box_, fmt};

glib::wrapper! {
    #[doc(alias = "GtkCellLayout")]
    pub struct CellLayout(Interface<ffi::GtkCellLayout, ffi::GtkCellLayoutIface>);

    match fn {
        type_ => || ffi::gtk_cell_layout_get_type(),
    }
}

impl CellLayout {
    pub const NONE: Option<&'static CellLayout> = None;
}

pub trait CellLayoutExt: 'static {
    #[doc(alias = "gtk_cell_layout_add_attribute")]
    fn add_attribute(&self, cell: &impl IsA<CellRenderer>, attribute: &str, column: i32);

    #[doc(alias = "gtk_cell_layout_clear")]
    fn clear(&self);

    #[doc(alias = "gtk_cell_layout_clear_attributes")]
    fn clear_attributes(&self, cell: &impl IsA<CellRenderer>);

    #[doc(alias = "gtk_cell_layout_get_area")]
    #[doc(alias = "get_area")]
    fn area(&self) -> Option<CellArea>;

    #[doc(alias = "gtk_cell_layout_get_cells")]
    #[doc(alias = "get_cells")]
    fn cells(&self) -> Vec<CellRenderer>;

    #[doc(alias = "gtk_cell_layout_pack_end")]
    fn pack_end(&self, cell: &impl IsA<CellRenderer>, expand: bool);

    #[doc(alias = "gtk_cell_layout_pack_start")]
    fn pack_start(&self, cell: &impl IsA<CellRenderer>, expand: bool);

    #[doc(alias = "gtk_cell_layout_reorder")]
    fn reorder(&self, cell: &impl IsA<CellRenderer>, position: i32);

    //#[doc(alias = "gtk_cell_layout_set_attributes")]
    //fn set_attributes(&self, cell: &impl IsA<CellRenderer>, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs);

    #[doc(alias = "gtk_cell_layout_set_cell_data_func")]
    fn set_cell_data_func(
        &self,
        cell: &impl IsA<CellRenderer>,
        func: Option<Box_<dyn Fn(&CellLayout, &CellRenderer, &TreeModel, &TreeIter) + 'static>>,
    );
}

impl<O: IsA<CellLayout>> CellLayoutExt for O {
    fn add_attribute(&self, cell: &impl IsA<CellRenderer>, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_cell_layout_add_attribute(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                attribute.to_glib_none().0,
                column,
            );
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::gtk_cell_layout_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn clear_attributes(&self, cell: &impl IsA<CellRenderer>) {
        unsafe {
            ffi::gtk_cell_layout_clear_attributes(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
            );
        }
    }

    fn area(&self) -> Option<CellArea> {
        unsafe {
            from_glib_none(ffi::gtk_cell_layout_get_area(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn cells(&self) -> Vec<CellRenderer> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_cell_layout_get_cells(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pack_end(&self, cell: &impl IsA<CellRenderer>, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_end(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                expand.into_glib(),
            );
        }
    }

    fn pack_start(&self, cell: &impl IsA<CellRenderer>, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_start(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                expand.into_glib(),
            );
        }
    }

    fn reorder(&self, cell: &impl IsA<CellRenderer>, position: i32) {
        unsafe {
            ffi::gtk_cell_layout_reorder(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    //fn set_attributes(&self, cell: &impl IsA<CellRenderer>, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_cell_layout_set_attributes() }
    //}

    fn set_cell_data_func(
        &self,
        cell: &impl IsA<CellRenderer>,
        func: Option<Box_<dyn Fn(&CellLayout, &CellRenderer, &TreeModel, &TreeIter) + 'static>>,
    ) {
        let func_data: Box_<
            Option<Box_<dyn Fn(&CellLayout, &CellRenderer, &TreeModel, &TreeIter) + 'static>>,
        > = Box_::new(func);
        unsafe extern "C" fn func_func(
            cell_layout: *mut ffi::GtkCellLayout,
            cell: *mut ffi::GtkCellRenderer,
            tree_model: *mut ffi::GtkTreeModel,
            iter: *mut ffi::GtkTreeIter,
            data: glib::ffi::gpointer,
        ) {
            let cell_layout = from_glib_borrow(cell_layout);
            let cell = from_glib_borrow(cell);
            let tree_model = from_glib_borrow(tree_model);
            let iter = from_glib_borrow(iter);
            let callback: &Option<
                Box_<dyn Fn(&CellLayout, &CellRenderer, &TreeModel, &TreeIter) + 'static>,
            > = &*(data as *mut _);
            if let Some(ref callback) = *callback {
                callback(&cell_layout, &cell, &tree_model, &iter)
            } else {
                panic!("cannot get closure...")
            };
        }
        let func = if func_data.is_some() {
            Some(func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn destroy_func(data: glib::ffi::gpointer) {
            let _callback: Box_<
                Option<Box_<dyn Fn(&CellLayout, &CellRenderer, &TreeModel, &TreeIter) + 'static>>,
            > = Box_::from_raw(data as *mut _);
        }
        let destroy_call4 = Some(destroy_func as _);
        let super_callback0: Box_<
            Option<Box_<dyn Fn(&CellLayout, &CellRenderer, &TreeModel, &TreeIter) + 'static>>,
        > = func_data;
        unsafe {
            ffi::gtk_cell_layout_set_cell_data_func(
                self.as_ref().to_glib_none().0,
                cell.as_ref().to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call4,
            );
        }
    }
}

impl fmt::Display for CellLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellLayout")
    }
}
