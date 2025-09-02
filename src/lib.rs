use mlua::{Function, Lua, Table};

#[mlua::lua_module(name = "libplunder")]
pub fn init(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set("add", Function::wrap(|(a, b): (u64, u64)| Ok(a + b)))?;
    Ok(table)
}
