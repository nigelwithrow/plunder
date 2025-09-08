use std::ops::{Deref, DerefMut};

mod utils;

#[derive(Debug, Clone, Copy)]
pub enum Sample<const CHANNELS: usize> {
    I16([i16; CHANNELS]),
    I24([[u8; 3]; CHANNELS]),
    I32([i32; CHANNELS]),
    F32([f32; CHANNELS]),
    F64([f64; CHANNELS]),
}

impl<const CHANNELS: usize> Sample<CHANNELS> {
    pub fn bit_depth(&self) -> u16 {
        match self {
            Sample::I16(_) => 16,
            Sample::I24(_) => 24,
            Sample::I32(_) => 32,
            Sample::F32(_) => 32,
            Sample::F64(_) => 64,
        }
    }

    // pub fn to_f64(&self) -> f64 {
    //     match self {
    //         Sample::I16(val) => *val as f64 / i16::MAX as f64,
    //         Sample::I24(bytes) => {
    //             // Use u32 for bit manipulation, then cast to i32
    //             let raw_value =
    //                 u32::from(bytes[0]) | (u32::from(bytes[1]) << 8) | (u32::from(bytes[2]) << 16);

    //             // Sign-extend if the most significant bit is set
    //             let sign_extended = if raw_value & 0x800000 != 0 {
    //                 (raw_value | 0xFF000000) as i32 // Now safe: operating on u32 first
    //             } else {
    //                 raw_value as i32
    //             };

    //             sign_extended as f64 / 8_388_608.0 // 2^23
    //         }
    //         Sample::I32(val) => *val as f64 / i32::MAX as f64,
    //         Sample::F32(val) => *val as f64,
    //         Sample::F64(val) => *val,
    //     }
    // }
}

pub trait Instrument<const CHANNELS: usize> {
    fn init(&self) -> Result<(), String>;
    fn get(&self, id: u32) -> Option<Sample<CHANNELS>>;
}

// pub trait AudioInstrument {
//     type Ping;
//     /// `None` indicates stream over
//     fn next(&mut self, input: Self::Ping) -> Option<Sample>;
// }

// Only for organisation purposes
// pub trait Plugin {
//     const NAME: &str;
//     fn to_lua(lua: &Lua) -> mlua::Result<impl IntoLua>;
// }

pub use mlua::prelude::*;
use utils::RegistryTransfer;

pub trait InstrumentFactory {
    // TODO let factory create more complex APIs (e.g.: `samp.wav {'..'}`)
    type Args: FromLuaMulti;
    type Instrument: Instrument<1> + Instrument<2> + LuaUserData;
    const NAME: &str;
    fn construct(args: Self::Args) -> LuaResult<Self::Instrument>;
}

#[derive(Debug)]
pub struct InstrumentWrapper<T>(T);
impl<T> InstrumentWrapper<T> {
    pub fn new(instrument: T) -> Self {
        InstrumentWrapper(instrument)
    }
}
impl<T> Deref for InstrumentWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for InstrumentWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for InstrumentWrapper<Box<T>> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
impl<T> AsMut<T> for InstrumentWrapper<Box<T>> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Instrument<1> + Instrument<2> + LuaUserData> LuaUserData for InstrumentWrapper<Box<T>> {
    fn register(registry: &mut LuaUserDataRegistry<Self>) {
        let mut registry = RegistryTransfer::new(registry);
        T::add_fields(&mut registry);
        T::add_methods(&mut registry);
    }
}
