use std::sync::Arc;

use mlua::prelude::*;

#[mlua::lua_module(name = "libplunder")]
pub fn init(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    // OfWav
    table.set(
        "ofWav",
        LuaFunction::wrap(|path: String| {
            of_wav::OfWav::load(path)
                .map(|instrument| -> Box<dyn types::BiInstrument> { Box::new(instrument) })
                .map_err(|err| LuaError::ExternalError(Arc::new(err)))
        }),
    )?;

    // P1
    let p1_tbl = lua.create_table()?;
    p1_tbl.set(
        "render",
        LuaFunction::wrap(|(config, sheet, instruments)| {
            Ok(p1::P1::render(config, sheet, instruments)
                .map_err(Into::<LuaError>::into)?
                .map(|instrument| -> Box<dyn types::BiInstrument> { Box::new(instrument) }))
        }),
    )?;
    table.set("p1", p1_tbl)?;

    Ok(table)
}
