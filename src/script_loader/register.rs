use crate::script::user_editable_config::{ConfigValue, UserEditConfig};
use crate::script::Script;
use crate::script::{lua_enter_point::LuaEntryPoint, storage::BoxedStorage};
use crate::script_loader::config::UserEditableConfigKind;
use crate::script_loader::ScriptRegister;

use crate::script::user_config_define::UserConfigDefine;
use std::rc::Rc;

impl<'lua> ScriptRegister<'lua> {
    pub fn to_script(&self, storage: &BoxedStorage) -> Script<'lua> {
        let script = Script::builder()
            .identity(self.identity.as_str())
            .name(self.name.as_deref().unwrap_or(self.identity.as_str()))
            .storage(storage)
            .configs(Rc::new(
                self.user_editable_config
                    .iter()
                    .map(|cfg| {
                        let key = Rc::clone(&cfg.identity);
                        let value = UserEditConfig::builder()
                            .kind(match &*cfg.kind {
                                UserEditableConfigKind::Switch(b) => ConfigValue::Switch(*b),
                                UserEditableConfigKind::Select(_, v) => {
                                    ConfigValue::Select(v.clone())
                                }
                                UserEditableConfigKind::Text(v) => ConfigValue::Text(v.clone()),
                            })
                            .build();
                        (key, Rc::new(value))
                    })
                    .collect(),
            ))
            .entry_point(
                LuaEntryPoint::builder()
                    .get_matched_scripts(self.verify_applicable_fn_name.clone())
                    .config_update(self.config_update_fn_name.clone())
                    .build(),
            )
            .build();

        script
    }

    pub fn to_config_define(&self) -> UserConfigDefine {
        UserConfigDefine::builder()
            .script_identity(&self.identity)
            .defines(
                self.user_editable_config
                    .iter()
                    .map(|def| {
                        let key = def.identity.clone();
                        let def = def.clone();
                        (key, def)
                    })
                    .collect(),
            )
            .build()
    }
}
