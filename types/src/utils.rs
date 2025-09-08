use std::marker::PhantomData;

use mlua::prelude::*;

/// Create a Lua User-data registry for the `Child` of a `Parent`
///
/// `RegistryTransfer<Parent, Child>`, provided `Parent` implements `AsRef<Child>` and
/// `AsMut<Child>`, takes in a `UserDataRegistry<Parent>` and provides implementations for
/// `UserDataFields<Child>` and `UserDataMethods<Parent>`
pub struct RegistryTransfer<'a, Parent, Child> {
    parent_registry: &'a mut LuaUserDataRegistry<Parent>,
    p: PhantomData<Child>,
}

impl<'a, Parent, Child> RegistryTransfer<'a, Parent, Child> {
    pub fn new(parent_registry: &'a mut LuaUserDataRegistry<Parent>) -> Self {
        RegistryTransfer {
            parent_registry,
            p: PhantomData,
        }
    }
}

impl<'a, Parent, Child> LuaUserDataFields<Child> for RegistryTransfer<'a, Parent, Child>
where
    Parent: AsRef<Child> + AsMut<Child>,
{
    fn add_field<V>(&mut self, name: impl Into<String>, value: V)
    where
        V: IntoLua + 'static,
    {
        self.parent_registry.add_field(name, value)
    }

    fn add_field_method_get<M, R>(&mut self, name: impl Into<String>, method: M)
    where
        M: Fn(&Lua, &Child) -> LuaResult<R> + mlua::MaybeSend + 'static,
        R: IntoLua,
    {
        self.parent_registry
            .add_field_method_get(name, move |lua, parent| (method)(lua, parent.as_ref()))
    }

    fn add_field_method_set<M, A>(&mut self, name: impl Into<String>, mut method: M)
    where
        M: FnMut(&Lua, &mut Child, A) -> LuaResult<()> + mlua::MaybeSend + 'static,
        A: FromLua,
    {
        self.parent_registry
            .add_field_method_set(name, move |lua, parent, value| {
                (method)(lua, parent.as_mut(), value)
            })
    }

    fn add_field_function_get<F, R>(&mut self, name: impl Into<String>, function: F)
    where
        F: Fn(&Lua, LuaAnyUserData) -> LuaResult<R> + mlua::MaybeSend + 'static,
        R: IntoLua,
    {
        self.parent_registry
            .add_field_function_get(name, move |lua, user_data| (function)(lua, user_data))
    }

    fn add_field_function_set<F, A>(&mut self, name: impl Into<String>, mut function: F)
    where
        F: FnMut(&Lua, LuaAnyUserData, A) -> LuaResult<()> + mlua::MaybeSend + 'static,
        A: FromLua,
    {
        self.parent_registry
            .add_field_function_set(name, move |lua, user_data, value| {
                (function)(lua, user_data, value)
            })
    }

    fn add_meta_field<V>(&mut self, name: impl Into<String>, value: V)
    where
        V: IntoLua + 'static,
    {
        self.parent_registry.add_meta_field(name, value)
    }

    fn add_meta_field_with<F, R>(&mut self, name: impl Into<String>, f: F)
    where
        F: FnOnce(&Lua) -> LuaResult<R> + 'static,
        R: IntoLua,
    {
        self.parent_registry.add_meta_field_with(name, f)
    }
}

impl<'a, Parent, Child> LuaUserDataMethods<Child> for RegistryTransfer<'a, Parent, Child>
where
    Parent: AsRef<Child> + AsMut<Child>,
{
    fn add_method<M, A, R>(&mut self, name: impl Into<String>, method: M)
    where
        M: Fn(&Lua, &Child, A) -> LuaResult<R> + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        self.parent_registry
            .add_method(name, move |lua, parent, value| {
                (method)(lua, parent.as_ref(), value)
            })
    }

    fn add_method_mut<M, A, R>(&mut self, name: impl Into<String>, mut method: M)
    where
        M: FnMut(&Lua, &mut Child, A) -> LuaResult<R> + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        self.parent_registry
            .add_method_mut(name, move |lua, parent, value| {
                (method)(lua, parent.as_mut(), value)
            })
    }

    fn add_function<F, A, R>(&mut self, name: impl Into<String>, function: F)
    where
        F: Fn(&Lua, A) -> LuaResult<R> + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        self.parent_registry.add_function(name, function)
    }

    fn add_function_mut<F, A, R>(&mut self, name: impl Into<String>, function: F)
    where
        F: FnMut(&Lua, A) -> LuaResult<R> + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        self.parent_registry.add_function_mut(name, function)
    }

    fn add_meta_method<M, A, R>(&mut self, name: impl Into<String>, method: M)
    where
        M: Fn(&Lua, &Child, A) -> LuaResult<R> + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        self.parent_registry
            .add_meta_method(name, move |lua, parent, value| {
                (method)(lua, parent.as_ref(), value)
            })
    }

    fn add_meta_method_mut<M, A, R>(&mut self, name: impl Into<String>, mut method: M)
    where
        M: FnMut(&Lua, &mut Child, A) -> LuaResult<R> + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        self.parent_registry
            .add_meta_method_mut(name, move |lua, parent, value| {
                (method)(lua, parent.as_mut(), value)
            })
    }

    fn add_meta_function<F, A, R>(&mut self, name: impl Into<String>, function: F)
    where
        F: Fn(&Lua, A) -> LuaResult<R> + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        self.parent_registry.add_meta_function(name, function)
    }

    fn add_meta_function_mut<F, A, R>(&mut self, name: impl Into<String>, function: F)
    where
        F: FnMut(&Lua, A) -> LuaResult<R> + mlua::MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        self.parent_registry.add_meta_function_mut(name, function)
    }
}
