use crate::script::lua_enter_point::LuaEntryPoint;
use crate::script::storage::{BoxedStorage, StorageManager};
use crate::script::user_editable_config::UserEditConfig;
use mlua::{IntoLua, UserData, UserDataFields, UserDataMethods};
use std::collections::BTreeMap;

use std::ops::Deref;
use std::rc::Rc;
use typed_builder::TypedBuilder;

use self::storage::Storage;

pub mod lua_enter_point;
pub mod storage;
pub mod user_config_define;
pub mod user_editable_config;

#[derive(Debug, TypedBuilder, Clone)]
/// a loaded script
pub struct Script<'lua> {
    #[builder(setter(transform = |identity:&str|Rc::from(String::from(identity).into_boxed_str())))]
    /// the unique identity of this script
    pub identity: Rc<str>,
    /// the script name for user using
    #[builder(setter(transform  = |name:&str| String::from(name)))]
    pub name: String,
    #[builder(setter(transform = |storage: &BoxedStorage| StorageManager(storage.clone())))]
    /// using for set/get local storage
    pub storage: StorageManager,
    /// user edit able configs
    pub configs: Rc<BTreeMap<Rc<str>, Rc<UserEditConfig>>>,

    /// entry points
    pub entry_point: Rc<LuaEntryPoint<'lua>>,
}

impl<'lua> Script<'lua> {
    pub fn get_config(&self) -> ScriptConfig {
        ScriptConfig {
            identity: self.identity.clone(),
            configs: self.configs.clone(),
            storage: self.storage.clone(),
        }
    }
}

pub struct ScriptConfig {
    identity: Rc<str>,
    configs: Rc<BTreeMap<Rc<str>, Rc<UserEditConfig>>>,
    storage: StorageManager,
}

impl UserData for ScriptConfig {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("identity", |vm, this| this.identity.into_lua(vm));
        fields.add_field_method_get("config", |vm, this| {
            let iter = this.configs.iter().map(|(k, v)| (k.deref(), Rc::clone(v)));
            vm.create_table_from(iter)
        });
    }
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("storageLoad", |_vm, this, key: String| {
            let ret = this
                .storage
                .load(&this.identity, key.as_str())
                .map_err(mlua::Error::external)?;

            Ok(String::from_utf8_lossy(&ret).to_string())
        });

        methods.add_method(
            "storageStore",
            |_vm, this, (key, value): (String, String)| {
                this.storage
                    .store(&this.identity, key.as_str(), value.as_bytes())
                    .map_err(mlua::Error::external)
            },
        );
    }
}
