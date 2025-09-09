//! `p1`, the flagship parser instrument included with Plunder

use std::{collections::HashMap, fmt, ops::Deref as _, str::FromStr, sync::Arc};

use types::*;

#[derive(Debug)]
pub enum P1Error {
    Lua(LuaError),
    Sheet,
    InstrumentUnknown(String),
    ArrangementMismatch(bool),
    UnboundInstrument(String),
}

impl fmt::Display for P1Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            P1Error::Lua(error) => error.fmt(f),
            P1Error::Sheet => write!(f, "Sheet error"),
            P1Error::InstrumentUnknown(name) => write!(
                f,
                "Instrument provided for \"{name}\" is of an unrecognized Lua type"
            ),
            P1Error::ArrangementMismatch(false) => write!(
                f,
                "Sheet provided is labelled but the instruments table is indexed"
            ),
            P1Error::ArrangementMismatch(true) => write!(
                f,
                "Sheet provided is indexed but the instruments table is labelled"
            ),
            P1Error::UnboundInstrument(name) => write!(
                f,
                "Sheet mentions instrument \"{name}\" which is not provided in instruments"
            ),
        }
    }
}

impl std::error::Error for P1Error {}

impl From<LuaError> for P1Error {
    fn from(value: LuaError) -> Self {
        P1Error::Lua(value)
    }
}

impl From<P1Error> for LuaError {
    fn from(value: P1Error) -> Self {
        match value {
            P1Error::Lua(error) => error,
            _ => LuaError::ExternalError(Arc::new(value)),
        }
    }
}

//
// Sheets
//
pub type SourceIndexList = Vec<Option<usize>>;

/// The kinds of sheets that the `p1` plugin can take
///
/// All entries are guaranteed to be strings of the same length
#[derive(Debug)]
pub enum Sheet {
    Labelled {
        /// NOTE: inclusive
        r#loop: (usize, usize),
        sheet: HashMap<String, SourceIndexList>,
    },
    Indexed {
        /// NOTE: inclusive
        r#loop: (usize, usize),
        sheet: Vec<SourceIndexList>,
    },
}

impl Sheet {
    const SEPARATOR: char = '|';
    const LOOP_START: char = '[';
    const LOOP_END: char = ']';
    const EMPTY: char = ' ';

    const PAT_ONE_SHOT: char = 'o';
    const PAT_RESTART: char = '[';
    const PAT_HALT: char = ']';
    const PAT_SUSTAIN: char = ' ';
    const PAT_UNPAUSE: char = '(';
    const PAT_PAUSE: char = ')';

    fn pat_to_source_index_list<S: AsRef<str>>(s: S) -> SourceIndexList {
        let mut on = false;
        let mut iota = 0;
        s.as_ref()
            .chars()
            .map(|c| match c {
                Self::PAT_ONE_SHOT | Self::PAT_RESTART => {
                    on = true;
                    iota = 1;
                    Some(0)
                }
                Self::PAT_SUSTAIN => on.then(|| {
                    iota += 1;
                    iota - 1
                }),
                Self::PAT_HALT => on.then(|| {
                    on = false;
                    let n = iota;
                    iota = 0;
                    n
                }),
                Self::PAT_UNPAUSE => {
                    on = true;
                    iota += 1;
                    Some(iota - 1)
                }
                Self::PAT_PAUSE => on.then(|| {
                    on = false;
                    iota += 1;
                    iota - 1
                }),
                _ => unreachable!(),
            })
            .collect()
    }

    pub fn r#loop(&self) -> &(usize, usize) {
        match self {
            Sheet::Labelled { r#loop, sheet: _ } => &r#loop,
            Sheet::Indexed { r#loop, sheet: _ } => &r#loop,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Sheet::Labelled { r#loop: _, sheet } => sheet.len(),
            Sheet::Indexed { r#loop: _, sheet } => sheet.len(),
        }
    }
}

impl FromStr for Sheet {
    type Err = ();

    fn from_str(sheet: &str) -> Result<Self, ()> {
        // Split a string with the given range inclusively, padding whitespace if the range exceeds the string bounds
        fn split_pad_inclusive(input: &str, start: usize, end: usize, c: &char) -> String {
            let input: Vec<_> = input.chars().collect();
            String::from_iter((start..=end).map(|i| input.get(i).unwrap_or(c)))
        }

        let mut lines = sheet.lines();
        let Some(first_line) = lines.next() else {
            return Err(());
        };

        // Collect lines with non-empty patterns so we can traverse them without consuming them
        //
        // TODO allow lines to have comments
        let lines: Vec<_> = lines
            .filter(|line| match line.find(Self::SEPARATOR) {
                None => line.len() > 0,
                Some(sep_column) => line.len() > sep_column + 1,
            })
            .collect();

        let last_column = lines.iter().map(|line| line.len()).max();
        let loop_start_column = first_line.find(Self::LOOP_START);
        let loop_end_column = first_line.find(Self::LOOP_END);

        // TODO parse entire line, not just the part within the loop, making it faster to reload loops when only the loop range has changed
        Ok(match first_line.find(Self::SEPARATOR) {
            // Indexed sheet
            None => {
                let loop_start = loop_start_column.unwrap_or(0);
                let loop_end = loop_end_column.unwrap_or(
                    last_column.expect("Entering this closure => last_column.is_some()") - 1,
                );
                Sheet::Indexed {
                    r#loop: (loop_start, loop_end),
                    sheet: lines
                        .into_iter()
                        .map(|line| split_pad_inclusive(line, loop_start, loop_end, &Self::EMPTY))
                        .map(Self::pat_to_source_index_list)
                        .collect(),
                }
            }
            // Labelled sheet
            Some(sep_column) => {
                let loop_start = loop_start_column.unwrap_or(sep_column + 1);
                let loop_end = loop_end_column.unwrap_or(
                    last_column.expect("Entering this closure => last_column.is_some()") - 1,
                );
                Sheet::Labelled {
                    r#loop: (loop_start, loop_end),
                    sheet: lines
                        .into_iter()
                        .map(|line| {
                            let label = line[0..sep_column].trim();
                            (
                                label.to_string(),
                                split_pad_inclusive(line, loop_start, loop_end, &Self::EMPTY),
                            )
                        })
                        .map(|(k, v)| (k, Self::pat_to_source_index_list(v)))
                        .collect(),
                }
            }
        })
    }
}

impl FromLua for Sheet {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let string = value
            .as_string()
            .ok_or(LuaError::RuntimeError("expected table".into()))?
            .to_string_lossy();
        // TODO HERE: proper error handling for Sheet
        Ok(Sheet::from_str(&string).map_err(|()| P1Error::Sheet)?)
    }
}

//
// Instruments
//
pub type DynInstrument = LuaUserDataRef<Box<dyn types::BiInstrument>>;

pub enum Instruments {
    Labelled(HashMap<String, DynInstrument>),
    Indexed(Vec<DynInstrument>),
}

impl Instruments {
    pub fn from_lua_pairs<E>(
        instruments: impl Iterator<Item = Result<(LuaValue, LuaValue), E>>,
    ) -> Result<Option<Self>, P1Error>
    where
        P1Error: From<E>,
    {
        use Instruments::*;

        #[inline(always)]
        fn lua_value_to_instrument(
            lua_value: LuaValue,
            key: Result<String, i64>,
        ) -> Result<DynInstrument, P1Error> {
            let LuaValue::UserData(user_data) = lua_value else {
                return Err(P1Error::InstrumentUnknown(
                    key.unwrap_or_else(|i| i.to_string()),
                ));
            };
            Ok(user_data.borrow::<Box<dyn BiInstrument>>()?)
        }

        let mut collection = None;
        for instrument in instruments {
            let (name, instrument) = instrument?;
            match (&collection, &name) {
                // Determine arrangement type of this instrument-table
                (None, LuaValue::String(_)) => collection = Some(Labelled(HashMap::new())),
                (None, LuaValue::Integer(_)) => collection = Some(Indexed(Vec::new())),
                (Some(_), _) => (),
                // Ignore
                // - number-indexed pairs for instrument-table determined to be labelled,
                // - string-indexed pairs for instrument-table determined to be indexed, and
                // - weird pairs with non-int non-string keys
                (None, _) => continue,
            }
            match (&mut collection, name) {
                // Determine arrangement type of this instrument-table
                // Relevant pair for this instrument-table
                (Some(Labelled(map)), LuaValue::String(s)) => {
                    _ = map.insert(
                        s.to_string_lossy(),
                        lua_value_to_instrument(instrument, Ok(s.to_string_lossy()))?,
                    )
                }
                // NOTE: ignores pair index (making it possible to have comments and stuff)
                (Some(Indexed(list)), LuaValue::Integer(i)) => {
                    list.push(lua_value_to_instrument(instrument, Err(i))?)
                }
                (None, _) => unreachable!("previous match ensures `collection` is initialized"),
                _ => unreachable!("previous match"),
            }
        }
        Ok(collection)
    }
}

impl FromLua for Instruments {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let table = value
            .as_table()
            .ok_or(LuaError::RuntimeError("expected table".into()))?;
        Ok(Instruments::from_lua_pairs(table.pairs())
            .map_err(Into::<LuaError>::into)?
            .unwrap_or(Instruments::Indexed(Vec::new())))
    }
}

//
// Config
//
#[derive(serde::Deserialize)]
pub struct Config {
    pub interval: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config { interval: 1000 }
    }
}

impl FromLua for Config {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        use serde::Deserialize as _;
        Config::deserialize(LuaDeserializer::new(value))
    }
}

//
// Rendered Buffer
//
#[derive(Debug)]
pub enum P1Buffer {
    Mono(Vec<Sample<1>>),
    Stereo(Vec<Sample<2>>),
}

impl P1Buffer {
    pub fn render(
        config: Config,
        sheet: Sheet,
        instruments: Instruments,
    ) -> Result<Option<Self>, P1Error> {
        let mut buffer = None;

        match (sheet, instruments) {
            (Sheet::Labelled { sheet, .. }, Instruments::Labelled(instruments)) => {
                for (name, pat) in sheet.iter() {
                    let size = pat.len() * config.interval;
                    let instrument = instruments
                        .get(name)
                        .ok_or_else(|| P1Error::UnboundInstrument(name.clone()))?;
                    let instrument: &Box<dyn types::BiInstrument> = &*instrument;
                    let instrument: &dyn types::BiInstrument = instrument.deref();
                    match (
                        Instrument::<2>::init(instrument),
                        Instrument::<1>::init(instrument),
                    ) {
                        (Ok(()), _) => {
                            match buffer {
                                Some(P1Buffer::Mono(_)) => {
                                    unimplemented!("upgrade mono buffer to stereo")
                                }
                                Some(P1Buffer::Stereo(_)) => (),
                                // Initialize buffer as stereo buffer
                                None => {
                                    buffer = Some(P1Buffer::Stereo(Vec::with_capacity(size * 32)))
                                }
                            }
                            let Some(P1Buffer::Stereo(buffer)) = &mut buffer else {
                                unreachable!("buf should be initialized as stereo at this point")
                            };

                            for i in 0..size {
                                buffer.insert(
                                    i,
                                    buffer.get(i).map(|_| todo!("blend sample")).unwrap_or(
                                        instrument.get(0).unwrap_or_else(|| todo!("empty sample")),
                                    ),
                                );
                            }
                        }
                        (_, Ok(())) => {}
                        (Err(_), Err(_)) => todo!(),
                    }
                }
            }
            (Sheet::Indexed { .. }, Instruments::Indexed(_)) => todo!(),
            _ => unreachable!(),
        }
        Ok(buffer)
    }
}

//
// P1, the Instrument
//
#[derive(Clone)]
pub struct P1(Arc<P1Buffer>);

impl P1 {
    pub fn render(
        config: Config,
        sheet: Sheet,
        instruments: Instruments,
    ) -> Result<Option<Self>, P1Error> {
        Ok(P1Buffer::render(config, sheet, instruments)?.map(|p1_buffer| P1(Arc::new(p1_buffer))))
    }
}

impl types::Instrument<1> for P1 {
    fn init(&self) -> Result<(), String> {
        // let mut inner = self.deref_();
        // match inner.buffer {
        //     Some(P1Buffer::Mono(_)) => Ok(()),
        //     Some(P1Buffer::Stereo(_)) => unimplemented!("downgrade stereo buffer"),
        //     None => match &inner.sheet {
        //         // No buffer but sheet has loop - allocate empty buffer of loop size
        //         //
        //         // TODO add warning here: sheet present, 0 instruments routed
        //         Some(sheet) => {
        //             let (loop_start, loop_end) = sheet.r#loop();
        //             inner.buffer = Some(P1Buffer::Mono(
        //                 (*loop_start..=*loop_end)
        //                     .map(|_| (0..32).map(|_| Sample::F32([0.])))
        //                     .flatten()
        //                     .collect(),
        //             ));
        //             Ok(())
        //         }
        //         None => Err("P1 can't generate any output without a sheet".to_string()),
        //     },
        // }
        Ok(())
    }

    fn get(&self, id: u32) -> Option<Sample<1>> {
        let P1Buffer::Mono(mono_buffer) = &*self.0 else {
            unreachable!("if init called, buffer should be initialized as mono")
        };
        mono_buffer.get(id as usize).copied()
    }
}

impl types::Instrument<2> for P1 {
    fn init(&self) -> Result<(), String> {
        // let mut inner = self.deref_();
        // match inner.buffer {
        //     Some(P1Buffer::Stereo(_)) => Ok(()),
        //     Some(P1Buffer::Mono(_)) => unimplemented!("upgrade stereo buffer"),
        //     None => match &inner.sheet {
        //         // No buffer but sheet has loop - allocate empty buffer of loop size
        //         //
        //         // TODO add warning here: sheet present, 0 instruments routed
        //         Some(sheet) => {
        //             let (loop_start, loop_end) = sheet.r#loop();
        //             inner.buffer = Some(P1Buffer::Stereo(
        //                 (*loop_start..=*loop_end)
        //                     .map(|_| (0..32).map(|_| Sample::F32([0., 0.])))
        //                     .flatten()
        //                     .collect(),
        //             ));
        //             Ok(())
        //         }
        //         None => Err("P1 can't generate any output without a sheet".to_string()),
        //     },
        // }
        Ok(())
    }

    fn get(&self, id: u32) -> Option<Sample<2>> {
        let P1Buffer::Stereo(stereo_buffer) = &*self.0 else {
            unreachable!("if init called, buffer should be initialized as stereo")
        };
        stereo_buffer.get(id as usize).copied()
    }
}

impl types::BiInstrument for P1 {}

impl LuaUserData for P1 {}

#[cfg(test)]
mod tests {
    #[test]
    fn pat_to_source_index_list() {
        fn pat_to_source_index_list(s: &str) -> Vec<isize> {
            super::Sheet::pat_to_source_index_list(s)
                .into_iter()
                .map(|s| s.map(|u| u as _).unwrap_or(-1))
                .collect()
        }
        assert_eq!(
            pat_to_source_index_list(r#"o   o   o   o   "#).as_slice(),
            &[0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3],
        );
        assert_eq!(
            pat_to_source_index_list(r#"  o   o   o   o "#).as_slice(),
            &[-1, -1, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1],
        );
        assert_eq!(
            pat_to_source_index_list(r#"oooooooooooooooo"#).as_slice(),
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        assert_eq!(
            pat_to_source_index_list(r#"[      ] [     ]"#).as_slice(),
            &[0, 1, 2, 3, 4, 5, 6, 7, -1, 0, 1, 2, 3, 4, 5, 6],
        );
        assert_eq!(
            pat_to_source_index_list(r#"[      ) (     ]"#).as_slice(),
            &[0, 1, 2, 3, 4, 5, 6, 7, -1, 8, 9, 10, 11, 12, 13, 14],
        );
        assert_eq!(
            pat_to_source_index_list(r#"[    ) )       ]"#).as_slice(),
            &[0, 1, 2, 3, 4, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        );
    }
}
