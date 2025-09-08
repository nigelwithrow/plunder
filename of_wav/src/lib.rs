use std::{fmt, path::Path, sync::Arc};

use types::Sample;

pub enum OfWav {
    Mono(Vec<Sample<1>>),
    Stereo(Vec<Sample<2>>),
}

#[derive(Debug)]
pub enum WavError {
    Hound(hound::Error),
    UnsupportedBitDepth(u16),
    UnsupportedNumChannels(u16),
}

impl fmt::Display for WavError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WavError::Hound(error) => error.fmt(f),
            WavError::UnsupportedBitDepth(n) => write!(f, "Unsupported bit-depth {n}"),
            WavError::UnsupportedNumChannels(n) => write!(f, "Unsupported number of channels {n}"),
        }
    }
}

impl std::error::Error for WavError {}

impl From<hound::Error> for WavError {
    fn from(value: hound::Error) -> Self {
        WavError::Hound(value)
    }
}

impl OfWav {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, WavError> {
        use itertools::Itertools as _;

        let mut reader = hound::WavReader::open(path)?;
        let depth = reader.spec().bits_per_sample;
        let channels = reader.spec().channels;
        Ok(match channels {
            1 => OfWav::Mono(match depth {
                16 => reader
                    .samples::<i16>()
                    .map(|s| s.map(|s| Sample::I16([s])))
                    .collect::<Result<_, _>>()?,
                24 => reader
                    .samples::<i32>()
                    .map(|s| s.map(|s32| Sample::I24([s32.to_be_bytes()[1..].try_into().unwrap()])))
                    .collect::<Result<_, _>>()?,
                32 => reader
                    .samples::<i32>()
                    .map(|s| s.map(|s| Sample::I32([s])))
                    .collect::<Result<_, _>>()?,
                n => return Err(WavError::UnsupportedBitDepth(n)),
            }),
            2 => OfWav::Stereo(match depth {
                16 => reader
                    .samples::<i16>()
                    .chunks(2)
                    .into_iter()
                    .map(|mut s| match (s.next().unwrap(), s.next().unwrap()) {
                        (Ok(s1), Ok(s2)) => Ok(Sample::I16([s1, s2])),
                        (Err(e), _) => Err(e),
                        (_, Err(e)) => Err(e),
                    })
                    .collect::<Result<_, _>>()?,
                24 => reader
                    .samples::<i32>()
                    .chunks(2)
                    .into_iter()
                    .map(|mut s| match (s.next().unwrap(), s.next().unwrap()) {
                        (Ok(s1), Ok(s2)) => Ok(Sample::I24([
                            s1.to_be_bytes()[1..].try_into().unwrap(),
                            s2.to_be_bytes()[1..].try_into().unwrap(),
                        ])),
                        (Err(e), _) => Err(e),
                        (_, Err(e)) => Err(e),
                    })
                    .collect::<Result<_, _>>()?,
                32 => reader
                    .samples::<i32>()
                    .chunks(2)
                    .into_iter()
                    .map(|mut s| match (s.next().unwrap(), s.next().unwrap()) {
                        (Ok(s1), Ok(s2)) => Ok(Sample::I32([s1, s2])),
                        (Err(e), _) => Err(e),
                        (_, Err(e)) => Err(e),
                    })
                    .collect::<Result<_, _>>()?,
                n => return Err(WavError::UnsupportedBitDepth(n)),
            }),
            n => return Err(WavError::UnsupportedNumChannels(n)),
        })
    }
}

impl types::Instrument<1> for OfWav {
    fn init(&self) -> Result<(), String> {
        // Of_wav will provide an implementation to interpolate stereo audio to mono
        Ok(())
    }

    fn get(&self, id: u32) -> Option<types::Sample<1>> {
        match self {
            OfWav::Mono(samples) => samples.get(id as usize).copied(),
            OfWav::Stereo(_) => unimplemented!("Interpolation of stereo audio to mono"),
        }
    }
}

impl types::Instrument<2> for OfWav {
    fn init(&self) -> Result<(), String> {
        // Of_wav will provide an implementation to interpolate mono audio to stereo
        Ok(())
    }

    fn get(&self, id: u32) -> Option<types::Sample<2>> {
        match self {
            OfWav::Mono(_) => unimplemented!("Interpolation of mono audio to stereo"),
            OfWav::Stereo(samples) => samples.get(id as usize).copied(),
        }
    }
}

impl types::LuaUserData for OfWav {}

pub struct OfWavFactory;

// impl UserData for OfWavFactory {
//     fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
//         methods.add_meta_function(MetaMethod::Call, |_, path: String| {
//             OfWav::new(path)
//                 .map(|of_wav| types::InstrumentWrapper::new(Box::new(of_wav)))
//                 .map_err(|err| mlua::Error::ExternalError(Arc::new(err)))
//         });
//     }
// }

impl types::InstrumentFactory for OfWavFactory {
    type Args = String;
    type Instrument = OfWav;
    const NAME: &str = "ofWav";

    fn construct(path: Self::Args) -> types::LuaResult<Self::Instrument> {
        OfWav::new(path).map_err(|err| types::LuaError::ExternalError(Arc::new(err)))
    }
    // fn to_lua(_: &mlua::Lua) -> mlua::Result<impl mlua::IntoLua> {
    //     Ok(OfWavFactory)
    // }
}
