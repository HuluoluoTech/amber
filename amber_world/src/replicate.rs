//! Replicate World States
//! World State Delta:
//! 1/ create new object
//! 2/ update current object
//! 3/ destroy current object
//!

#[derive(Debug, PartialEq)]
pub enum ReplicationAction {
    RaCreate,
    RaUpdate,
    RaDestroy,
    RaMax,
}

pub trait Replication {
    fn create(&mut self);
    fn update(&mut self);
    fn destroy(&mut self);
}

// using bits field to represent mouse properties
pub enum MouseStatusProperties {
    MspName = 1 << 0,
    MspLegCount = 1 << 1,
    MspHeadCount  = 1 << 2,
    MspHealth = 1 << 3,
    MspMax,
}
