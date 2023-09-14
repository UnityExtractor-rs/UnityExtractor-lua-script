use crate::UnityBundle;
use mlua::{
    FromLua, Function, Lua, Table,
    Value::{self, Nil},
};
use std::{collections::HashMap, rc::Rc};

use crate::script::ScriptConfig;
use typed_builder::TypedBuilder;

#[derive(Debug)]
/// a entry of a sub script
pub struct SubScriptEntry<'lua> {
    /// the name of this sub script for user reading
    pub name: String,
    /// the entry point of this sub script
    ///
    ///this function has 3 args
    /// 1. [Script](super::Script) the script itself can read config or other message from this
    /// 2. [UnityBundle](UnityBundle) the unity bundle itself
    /// 3. [GlobalObjectManager] a manager which can using for save file, create script Fake Object and so on
    ///
    /// no return value
    ///
    ///in this function , the script can  
    pub entry: Function<'lua>,
}

impl<'lua> SubScriptEntry<'lua> {
    pub fn call(
        &self,
        script: ScriptConfig,
        bundle: UnityBundle,
        _manager: (),
    ) -> mlua::Result<()> {
        let func = self.entry.clone();
        let arg = (script, bundle, Nil);
        func.call(arg)?;
        Ok(())
    }
}

impl<'lua> FromLua<'lua> for SubScriptEntry<'lua> {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let table = Table::from_lua(value, lua)?;
        Ok(Self {
            name: table.get("name")?,
            entry: table.get("entry")?,
        })
    }
}

#[derive(Debug, TypedBuilder)]
pub struct LuaEntryPoint<'lua> {
    #[builder(default)]
    /// a lua function which will be call when user edit the config
    /// the config will be auto store to [BoxedStorage](super::storage::BoxedStorage),
    /// this may cause script state change,
    /// if not provide, will just update config to storage
    config_update: Option<Function<'lua>>,
    /// a lua function to determine whether a special [UnityBundle](UnityBundle) can be applied this [Script](super::Script) and
    /// witch sub scripts can be apply
    ///
    /// - if not support this [UnityBundle], the function will return [nil](lua::Value::Nil)
    /// - otherwise, return `Map<identity, SubScriptEntry>`, the key is the identity of the sub script and the value is [SubScriptEntry]
    get_matched_scripts: Function<'lua>,
}

impl<'lua> LuaEntryPoint<'lua> {
    pub fn update_config<'s, I>(&self, vm: &'lua Lua, updated_configs: I) -> mlua::Result<()>
    where
        I: IntoIterator<Item = (&'s str, &'s str)>,
    {
        if let Some(update) = &self.config_update {
            let updated = vm.create_table_from(updated_configs)?;
            update.call::<_, ()>(updated)?
        }
        Ok(())
    }

    pub fn get_matched_scripts(
        &self,
        bundle: UnityBundle,
    ) -> mlua::Result<Option<HashMap<String, Rc<SubScriptEntry<'lua>>>>> {
        let entry = self
            .get_matched_scripts
            .call::<_, Option<HashMap<String, SubScriptEntry<'lua>>>>(bundle)?
            .map(|map| map.into_iter().map(|(k, v)| (k, Rc::new(v))).collect());
        Ok(entry)
    }
}
