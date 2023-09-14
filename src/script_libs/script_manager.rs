use crate::script_libs::local_storage::LocalStorage;
use crate::unity_object::UnityObject;
use mlua::{FromLua, Function, Lua, Table, UserData, UserDataFields, Value};
use std::collections::HashMap;
use std::rc::Rc;

pub struct ScriptManager<'lua> {
    name: String,

    // functions
    on_user_config_update: Function<'lua>,
    verify_applicable: Function<'lua>,
    entry_points: Table<'lua>,

    // storage
    local_storage: Rc<LocalStorage>,
}

pub struct UpdateConfig {
    idx: String,
    value: String,
}

impl UserData for UpdateConfig {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("idx", |_, this| Ok(this.idx.clone()));
        fields.add_field_method_get("value", |_, this| Ok(this.value.clone()));
    }
}

pub struct ScriptEntryPoint {
    name: String,
    entry_point: String,
}

impl<'lua> FromLua<'lua> for ScriptEntryPoint {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(tb) => Ok(Self {
                name: tb.get("name")?,
                entry_point: tb.get("entryPoint")?,
            }),
            _ => Err(mlua::Error::ToLuaConversionError {
                from: value.type_name(),
                to: "ScriptEntryPoint",
                message: None,
            }),
        }
    }
}

impl<'lua> ScriptManager<'lua> {
    /// emit when user update the user editable config
    /// when first init , this function will be call for all config
    pub fn update_config<I>(&self, lua: &Lua, updated_configs: I) -> mlua::Result<()>
    where
        I: IntoIterator<Item = UpdateConfig>,
    {
        let table = lua.create_sequence_from(updated_configs)?;
        self.on_user_config_update
            .call((table, self.local_storage.clone()))?;
        Ok(())
    }

    /// check whether this script is usable on provide unity object
    ///
    ///1. if support this object, return a set of function entry point with its name,
    ///2. if not support, return empty map
    ///
    /// TODO: Define the unity object
    pub fn get_operates(
        &self,
        unity_object: UnityObject,
    ) -> mlua::Result<Option<HashMap<String, ScriptEntryPoint>>> {
        self.verify_applicable
            .call((unity_object, self.local_storage.clone()))
    }

    pub fn execute_operate(
        &self,
        entry: &ScriptEntryPoint,
        unity_object: UnityObject,
    ) -> mlua::Result<()> {
        let func: Function = self.entry_points.get(&*entry.entry_point)?;
        func.call(unity_object)?;
        Ok(())
    }
}
