use mlua::{IntoLua, Lua, Table};

/// `p1`, the flagship parser included with Plunder
mod p1;

#[mlua::lua_module(name = "libplunder")]
pub fn init(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set(p1::P1Factory::NAME, p1::P1Factory::to_lua(lua)?)?;
    Ok(table)
}

trait Plugin {
    const NAME: &str;
    fn to_lua(lua: &Lua) -> mlua::Result<impl IntoLua>;
}
