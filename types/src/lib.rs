pub mod registry_transfer;

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

pub use mlua::prelude::*;
pub use mlua::serde::Deserializer as LuaDeserializer;
pub use mlua::serde::Serializer as LuaSerializer;

// pub trait InstrumentFactory {
//     // TODO let factory create more complex APIs (e.g.: `samp.wav {'..'}`)
//     type Args: FromLuaMulti;
//     const NAME: &str;
//     fn construct(args: Self::Args) -> LuaResult<Box<dyn BiInstrument>>;
// }

pub trait BiInstrument: Instrument<1> + Instrument<2> {}

impl LuaUserData for Box<dyn BiInstrument> {}

// pub struct InstrumentWrapper(pub Box<dyn BiInstrument>);
// pub struct InstrumentWrapper<T>(pub DynInstrumentWrapper, PhantomData<T>);

// impl<T> InstrumentWrapper<T>
// where
//     T: BiInstrument + Clone + 'static,
// {
//     pub fn new(instrument: T) -> Self {
//         InstrumentWrapper(DynInstrumentWrapper(Box::new(instrument)), PhantomData)
//     }
// }
// // impl<T> Deref for InstrumentWrapper<T> {
// //     type Target = T;
// //     fn deref(&self) -> &Self::Target {
// //         &self.0
// //     }
// // }
// // impl<T> DerefMut for InstrumentWrapper<T> {
// //     fn deref_mut(&mut self) -> &mut Self::Target {
// //         &mut self.0
// //     }
// // }

// impl<T> Instrument<1> for InstrumentWrapper<T> {
//     fn init(&self) -> Result<(), String> {
//         <_ as Instrument<1>>::init(&*self.0.0)
//     }
//     fn get(&self, id: u32) -> Option<Sample<1>> {
//         self.0.0.get(id)
//     }
// }

// impl<T> Instrument<2> for InstrumentWrapper<T> {
//     fn init(&self) -> Result<(), String> {
//         <_ as Instrument<2>>::init(&*self.0.0)
//     }
//     fn get(&self, id: u32) -> Option<Sample<2>> {
//         self.0.0.get(id)
//     }
// }

// impl<T> BiInstrument for InstrumentWrapper<T> {}

// // impl<T> AsRef<T> for InstrumentWrapper<T> {
// //     fn as_ref(&self) -> &T {
// //         &self.0
// //     }
// // }
// // impl<T> AsMut<T> for InstrumentWrapper<T> {
// //     fn as_mut(&mut self) -> &mut T {
// //         &mut self.0
// //     }
// // }

// impl<T> AsRef<Box<dyn BiInstrument>> for InstrumentWrapper<T> {
//     fn as_ref(&self) -> &Box<dyn BiInstrument> {
//         &self.0.0
//     }
// }

// impl<T> AsMut<Box<dyn BiInstrument>> for InstrumentWrapper<T> {
//     fn as_mut(&mut self) -> &mut Box<dyn BiInstrument> {
//         &mut self.0.0
//     }
// }

// impl<T: Instrument<1> + Instrument<2> + LuaUserData> LuaUserData for InstrumentWrapper<T> {
//     fn register(registry: &mut LuaUserDataRegistry<Self>) {
//         let mut registry = RegistryTransfer::new(registry);
//         T::add_fields(&mut registry);
//         T::add_methods(&mut registry);
//     }
// }

// pub struct DynInstrumentWrapper(Box<dyn BiInstrument>);

// impl LuaUserData for DynInstrumentWrapper {
//     fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {}

//     fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {}

//     fn register(registry: &mut LuaUserDataRegistry<Self>) {
//         Self::add_fields(registry);
//         Self::add_methods(registry);
//     }
// }
