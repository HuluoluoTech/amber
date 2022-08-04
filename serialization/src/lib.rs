// import the generated code
#[allow(dead_code, unused_imports)]
#[path = "./game_generated.rs"]
mod game_generated;

pub use game_generated::*;

pub fn serli() {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);


}

