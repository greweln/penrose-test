#[macro_use]
extern crate penrose;

mod hooks;
use hooks::StartupScript;
mod consts;
use consts::{Conn, FG, FONT};

use penrose::{
    core::{config::Config, hooks::Hooks},
    draw::{
        Color, TextStyle,
    },
    logging_error_handler,
    xcb::{new_xcb_backed_window_manager},
    Result,
};
use simplelog::{LevelFilter, SimpleLogger};
use std::collections::HashMap;
use std::convert::TryFrom;

fn main() -> Result<()> {
    //////////////////////////
    //      LOGGING         //
    //////////////////////////
    SimpleLogger::init(LevelFilter::Debug, simplelog::Config::default())
        .expect("failed to init logging");

    //////////////////////////
    //       CONFIG         //
    //////////////////////////
    let config = Config::default()
        .builder()
        .workspaces(vec!["1", "2", "3", "4", "5"])
        .floating_classes(consts::floating_classes())
        .focused_border("#9b9b9b")?
        .unfocused_border("#1d1f21")?
        .gap_px(10)
        .bar_height(30)
        .layouts(vec![consts::my_layout()])
        .build()
        .unwrap();

    let _style = TextStyle {
        font: FONT.to_string(),
        point_size: 12,
        // fg: widgets,
        // fg: active screen with window
        // fg: workspace with window
        fg: Color::try_from(FG)?, // c5c8c6
        // bg: inactive workspaces
        // bg: widgets
        bg: Some(Color::try_from("#1d1f21")?),
        padding: (2.0, 2.0),
    };


    /////////////////////////
    //         HOOKS       //
    /////////////////////////
    let hooks: Hooks<Conn> = vec![
        Box::new(StartupScript::new("/home/sergiu/scripts/gwm_startup.sh")),
    ];

    /////////////////////////
    //      KEYBINDINGS    //
    /////////////////////////
    let key_bindings = gen_keybindings! {
    };

    let mut wm = new_xcb_backed_window_manager(config, hooks, logging_error_handler())?;
    //bar.startup(&mut wm)?;
    wm.grab_keys_and_run(key_bindings, HashMap::new())?;

    Ok(())
}
