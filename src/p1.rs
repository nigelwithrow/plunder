// TODO: let instruments be routed both eagerly (when passed after sheet) or lazily (when passed before sheet)
use std::{
    collections::HashMap,
    iter::repeat_n,
    sync::{Arc, Mutex, MutexGuard},
};

use mlua::{IntoLua, Lua, Table, UserData, Value};

/// The kinds of sheets that the `p1` plugin can take
///
/// All entries are guaranteed to be strings of the same length
#[derive(Debug)]
enum SheetKind<T> {
    Labelled(HashMap<String, T>),
    Indexed(Vec<T>),
}

#[derive(Default)]
struct Inner {
    sheet: Option<SheetKind<String>>,
}

/// Split a string with the given range inclusively, padding whitespace if the range exceeds the string bounds
fn split_pad_inclusive(input: &str, start: usize, end: usize, c: &char) -> String {
    let input: Vec<_> = input.chars().collect();
    String::from_iter((start..=end).map(|i| input.get(i).unwrap_or(c)))
}

impl Inner {
    const SEPARATOR: char = '|';
    const LOOP_START: char = '[';
    const LOOP_END: char = ']';
    const EMPTY: char = ' ';

    fn parse(&mut self, sheet: &str) -> mlua::Result<()> {
        // TODO: allow extending sheet by repeated sheet calls instead of overwriting the previous one
        self.sheet = None;
        let mut lines = sheet.lines();
        let Some(first_line) = lines.next() else {
            return Ok(());
        };
        println!("`{first_line}`");

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
        let new_sheet = match first_line.find(Self::SEPARATOR) {
            // Indexed sheet
            None => SheetKind::Indexed(
                lines
                    .into_iter()
                    .map(|line| {
                        println!(">>`{line}`");
                        split_pad_inclusive(
                            line,
                            loop_start_column.unwrap_or(0),
                            loop_end_column.unwrap_or(
                                last_column
                                    .expect("Entering this closure => last_column.is_some()")
                                    - 1,
                            ),
                            &Self::EMPTY,
                        )
                    })
                    .collect(),
            ),
            // Labelled sheet
            Some(sep_column) => SheetKind::Labelled(
                lines
                    .into_iter()
                    .map(|line| {
                        let label = line[0..sep_column].trim();
                        (
                            label.to_string(),
                            split_pad_inclusive(
                                line,
                                loop_start_column.unwrap_or(sep_column + 1),
                                loop_end_column.unwrap_or(
                                    last_column
                                        .expect("Entering this closure => last_column.is_some()")
                                        - 1,
                                ),
                                &Self::EMPTY,
                            ),
                        )
                    })
                    .collect(),
            ),
        };
        self.sheet = Some(new_sheet);
        Ok(())
    }
}

#[derive(Clone)]
pub struct P1(Arc<Mutex<Inner>>);

impl P1 {
    fn new() -> Self {
        P1(Arc::new(Mutex::new(Inner::default())))
    }

    fn deref_(&self) -> MutexGuard<Inner> {
        self.0.lock().unwrap()
    }
}

impl UserData for P1 {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("instruments", |_, this, instruments: Table| {
            let count = instruments.pairs::<Value, Value>().count();
            println!("{} instruments attached", count);
            Ok(this.clone())
        });

        methods.add_method("sheet", |_, this, sheet: String| {
            this.deref_().parse(&sheet)?;
            Ok(this.clone())
        });
    }
    // fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {}
}

/// Factory to construct new `P1`s using `.new` in Lua
pub struct P1Factory;

impl UserData for P1Factory {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("new", |_, _| Ok(P1::new()));
    }
}

impl crate::Plugin for P1Factory {
    const NAME: &str = "p1";

    fn to_lua(_lua: &Lua) -> mlua::Result<impl IntoLua> {
        Ok(P1Factory)
    }
}
