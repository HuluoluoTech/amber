// import the generated code
#[allow(dead_code, unused_imports)]
#[path = "../schema/game_generated.rs"]
mod game_generated;

pub use game_generated::*;

pub fn serli() {
    let mut _builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
}

