#![allow(unused_imports)]

mod anchor_layout_example;
mod button;
pub use anchor_layout_example::AnchorLayoutExample;
pub use button::CustomButton;
use pax_lang::api::*;
use pax_lang::*;
use pax_std::components::Stacker;
use pax_std::primitives::*;
use pax_std::types::text::*;
use pax_std::types::*;

#[derive(Pax)]
#[main]
#[file("lib.pax")]
pub struct Example {}

impl Example {
    pub fn handle_did_mount(&mut self, _ctx: RuntimeContext) {
        pax_lang::log("Mounted Main!");
    }
}
