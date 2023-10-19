#![allow(unused_imports)]

mod anchor_layout_example;
mod button;
mod marker;
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
pub struct Example {
    pub val: Property<Size>,
}

impl Example {
    pub fn handle_did_mount(&mut self, _ctx: RuntimeContext) {
        pax_lang::log("Mounted Main!");
        self.val.set(Size::Percent(30.into()));
    }

    pub fn click9(&mut self, _ctx: RuntimeContext, _args: ArgsClick) {
        self.val.set(Size::Percent(10.into()));
    }

    pub fn click7(&mut self, _ctx: RuntimeContext, _args: ArgsClick) {
        self.val.set(Size::Percent(30.into()));
    }

    pub fn click5(&mut self, _ctx: RuntimeContext, _args: ArgsClick) {
        self.val.set(Size::Percent(50.into()));
    }

    pub fn click3(&mut self, _ctx: RuntimeContext, _args: ArgsClick) {
        self.val.set(Size::Percent(70.into()));
    }

    pub fn click1(&mut self, _ctx: RuntimeContext, _args: ArgsClick) {
        self.val.set(Size::Percent(90.into()));
    }
}
