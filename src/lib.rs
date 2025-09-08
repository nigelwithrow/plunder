use mlua::{Function, Lua, Table, UserData};

#[mlua::lua_module(name = "libplunder")]
pub fn init(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    register_instrument_factory::<of_wav::OfWavFactory>(&table)?;
    register_instrument_factory::<p1::P1Factory>(&table)?;
    Ok(table)
}

fn register_instrument_factory<T>(table: &Table) -> mlua::Result<()>
where
    T: types::InstrumentFactory + 'static,
    T::Instrument: UserData,
{
    table.set(
        T::NAME,
        Function::wrap(|args: T::Args| {
            T::construct(args).map(|instrument| types::InstrumentWrapper::new(Box::new(instrument)))
        }),
    )
}
