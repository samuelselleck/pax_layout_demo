#![allow(unused_imports)]

use super::marker::Marker;
use pax_lang::api::*;
use pax_lang::*;
use pax_std::components::Stacker;
use pax_std::primitives::*;
use pax_std::types::text::*;
use pax_std::types::*;

#[derive(Pax)]
#[file("anchor_layout_example.pax")]
pub struct AnchorLayoutExample {
    pub size: Property<Size>,
    pub p_x: Property<Size>,
    pub p_y: Property<Size>,
    pub a_x: Property<Size>,
    pub a_y: Property<Size>,
}

impl AnchorLayoutExample {
    pub fn handle_did_mount(&mut self, _ctx: RuntimeContext) {
        pax_lang::log("Mounted Button!");
    }

    pub fn clicked(&mut self, _ctx: RuntimeContext, _args: ArgsClick) {}
}
