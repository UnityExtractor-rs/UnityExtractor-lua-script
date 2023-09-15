mod error;
mod sub_script_manager;

use crate::script::storage::BoxedStorage;
use crate::script::user_config_define::UserConfigDefine;
use crate::script::Script;
use crate::script_libs::ScriptRegister;
use crate::UnityBundle;
pub use error::ScriptError;
use mlua::{Function, Lua, LuaOptions, StdLib};

use std::path::Path;

use std::ptr::NonNull;
pub use sub_script_manager::{SubScriptKeys, SubScriptManger};

struct ScriptItem {
    script: Script<'static>,
    define: UserConfigDefine,
}

pub struct ScriptManager {
    storage: BoxedStorage,
    script: Option<ScriptItem>,
    vm: NonNull<Lua>,
}

impl Drop for ScriptManager {
    fn drop(&mut self) {
        if let Some(ScriptItem { script, define }) = self.script.take() {
            drop(script);
            drop(define);
        }
        let v = unsafe { Box::from_raw(self.vm.as_ptr()) };
        drop(v)
    }
}

impl ScriptManager {
    pub fn new(opt: LuaOptions, storage: BoxedStorage) -> Result<Self, ScriptError> {
        Ok(Self {
            vm: Box::leak(Box::new(Lua::new_with(StdLib::ALL_SAFE, opt)?)).into(),
            storage,
            script: None,
        })
    }

    pub fn load_entry(&mut self, entry: &impl AsRef<Path>) -> Result<(), ScriptError> {
        let vm = unsafe { self.vm.as_ref() };
        // load script
        vm.load(entry.as_ref()).exec()?;
        println!("init ");
        // register script
        let init_func: Function = vm.globals().get("InitScript")?;
        let register: ScriptRegister = init_func.call(())?;

        let script = register.to_script(&self.storage);
        let define = register.to_config_define();

        println!("{script:?}");

        (self.script) = Some(ScriptItem { script, define });
        Ok(())
    }

    pub fn init_script(&self) -> Result<(), ScriptError> {
        let ScriptItem { script, define } =
            self.script.as_ref().ok_or(ScriptError::ScriptNotLoad)?;

        // check config item exist,
        for (key, config) in script.configs.iter() {
            if self.storage.contains_key(&script.identity, key)? {
                continue;
            } else {
                define.update(&script.storage, &script.identity, config.kind.clone())?;
            }
        }
        Ok(())
    }

    pub fn get_match_scripts(
        &self,
        bundle: UnityBundle,
    ) -> Result<SubScriptManger<'_>, ScriptError> {
        let script = &self
            .script
            .as_ref()
            .ok_or(ScriptError::ScriptNotLoad)?
            .script;
        let result = script.entry_point.get_matched_scripts(bundle)?;
        Ok(SubScriptManger {
            sub_scripts: result,
            script,
        })
    }
}
