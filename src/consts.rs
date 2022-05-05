use penrose::{
    core::layout::{side_stack, Layout, LayoutConf},
    XcbConnection,
};

pub const FG: &str = "#bcbcbc";
pub const FONT: &str = "DejaVu Sans Mono Nerd Font Complete Mono";

pub type Conn = XcbConnection;

pub fn floating_classes() -> Vec<&'static str> {
    vec![
        "dialog",
        "confirm",
        "error",
        "file_progress",
        "notification",
        "pinentry-gtk-2",
        "File upload",
        "Save As",
    ]
}
pub fn my_layout() -> Layout {
    Layout::new("[side]", LayoutConf::default(), side_stack, 1, 0.6)
}
