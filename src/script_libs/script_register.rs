use crate::script_libs::config_manage::UserEditableConfig;
use crate::script_libs::error::InitScriptError;
use mlua::{FromLua, Function, Table, UserData, UserDataFields, UserDataMethods};

#[derive(Debug, Clone, FromLua, Default)]
pub struct ScriptRegister {
    script_name: Option<String>,
    user_editable_config: Vec<UserEditableConfig>,
    config_update_fn_name: Option<String>,
    verify_applicable_fn_name: Option<String>,
    script_entry_point: Option<String>,
}

impl UserData for ScriptRegister {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {}
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut::<_, String, _>("name", |_vm, this, name| {
            println!("enter");
            if let Some(s) = &this.script_name {
                Err(mlua::Error::external(InitScriptError::ScriptNameHasSet(
                    s.to_string(),
                )))
            } else {
                this.script_name = Some(name);
                Ok(())
            }
        });
        methods.add_method_mut("addUserConfig", |_vm, this, config: UserEditableConfig| {
            this.user_editable_config.push(config);
            Ok(())
        });

        methods.add_method_mut("configUpdateCallback", |_, this, name| {
            this.config_update_fn_name = Some(name);
            Ok(())
        });

        methods.add_method_mut("verifyApplicable", |_, this, name| {
            this.verify_applicable_fn_name = Some(name);
            Ok(())
        });
        methods.add_method_mut("entryPoint", |_, this, value| {
            this.script_entry_point = Some(value);
            Ok(())
        })
    }
}
