// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use std::fmt;

glib::wrapper! {
    pub struct RepeatingRadialGradientNode(Object<ffi::GskRepeatingRadialGradientNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_repeating_radial_gradient_node_get_type(),
    }
}

impl RepeatingRadialGradientNode {}

impl fmt::Display for RepeatingRadialGradientNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RepeatingRadialGradientNode")
    }
}
