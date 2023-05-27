//! Module for everything that pertains to maps
//!
//! This modules includes:
//! - A plugin to setup the required systems and loaders
//! - An asset loader to load Tiled map files
//! - A loader to load maps into the world
//! - A state system to handle the flow of map loading and unloading
//! - A bunch of components to mark or hold info for the various different tiles

pub mod asset;
pub mod loader;
pub mod plugin;
pub mod state;
pub mod tiles;
pub mod util;
