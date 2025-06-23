pub mod dx11;
pub mod engine;
pub mod object;
pub mod pack;
pub mod render_list;
#[cfg(feature = "goggles")]
pub mod goggles;
#[deprecated = "crate::resources"]
pub(crate) use crate::resources;

pub use engine::Engine;

const M_TO_UNIT: f32 = 3.28084;
const MILE_TO_FT: f32 = 5280.0;
//pub const MAX_DEPTH: f32 = 10_000.0 / (M_TO_UNIT * 2.0);
//pub const MIN_DEPTH: f32 = (M_TO_UNIT * 2.0) / 10.0;
pub const MAX_DEPTH: f32 = max_depth_from_scale(1.0 / M_TO_UNIT);
pub const MIN_DEPTH: f32 = min_depth_from_scale(1.0 / M_TO_UNIT);

pub const fn min_depth_from_scale(factor: f32) -> f32 {
    // TODO: this is actually more like... base around 0.5 * sqrt(map_width * map_height)
    // windswept haven is ~0.51 and square (30720^2) with ratio 1unit=24 inches
    // example: mistlock: ~0.25 because it's square (12288^2) but compressed where 1unit=12inches
    // wizards tower is 33792x18432 and ~0.64, which is like...  0.51/(sqrt(33792*18432)/30720) (or maybe 1/(18432/sqrt(18432*33792)*2) but might be off without unit conversions idk)
    // scaling factor is likely https://wiki.guildwars2.com/wiki/API:1/event_details#Coordinate_recalculation
    // so sqrt(scalex^2+scaley)=0.675 where scalex=(len-2433792/2)/33792.., scaley=(len-18432/2)/(18432); and maybe len=576=2ft?
    // 0.1*24*sqrt(0.5)*f2m=0.5172 for windswept, is compelling...
    //10.0 * factor / 12.0 // with 1x or 2x scale idk
    //factor * (2.0 + 1.0 / 12.0) // 0.6349
    //MILE_TO_FT * 12.0 / 100_000.0 // 0.6336
    factor * 2.1 // 0.64
    //MILE_TO_FT * M_TO_UNIT / 2500.0 // 0.643
}

pub const fn max_depth_from_scale(factor: f32) -> f32 {
    //10_000.0 / factor * 10.0 / 12.0 // with 2x scale
    //1_000.0 * factor * 10.0 / 6.0 // with 2x scale
    //10_000.0 * factor * 10.0 / 24.0 // with 1x scale
    //factor * MILE_TO_FT
    (MILE_TO_FT * 10.0 / 11.0) * factor // 1463.04
    //min_depth_from_scale(factor) * 2400.0
}

#[cfg(not(feature = "goggles"))]
pub const fn max_depth() -> f32 {
    MAX_DEPTH
}
#[cfg(not(feature = "goggles"))]
pub const fn min_depth() -> f32 {
    MIN_DEPTH
}

#[cfg(feature = "goggles")]
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(feature = "goggles")]
pub static MAX_DEPTH_: AtomicU32 = AtomicU32::new(
    u32::from_le_bytes(
        MAX_DEPTH
            .to_le_bytes()
    )
);

#[cfg(feature = "goggles")]
pub fn max_depth() -> f32 {
    f32::from_le_bytes(
        MAX_DEPTH_.load(Ordering::Relaxed)
            .to_le_bytes()
    )
}

#[cfg(feature = "goggles")]
pub fn set_max_depth(v: f32) {
    let v = u32::from_le_bytes(v.to_le_bytes());
    MAX_DEPTH_.store(v, Ordering::Relaxed)
}

#[cfg(feature = "goggles")]
pub static MIN_DEPTH_: AtomicU32 = AtomicU32::new(
    u32::from_le_bytes(
        MIN_DEPTH
            .to_le_bytes()
    )
);

#[cfg(feature = "goggles")]
pub fn min_depth() -> f32 {
    f32::from_le_bytes(
        MIN_DEPTH_.load(Ordering::Relaxed)
            .to_le_bytes()
    )
}

#[cfg(feature = "goggles")]
pub fn set_min_depth(v: f32) {
    let v = u32::from_le_bytes(v.to_le_bytes());
    MIN_DEPTH_.store(v, Ordering::Relaxed)
}
