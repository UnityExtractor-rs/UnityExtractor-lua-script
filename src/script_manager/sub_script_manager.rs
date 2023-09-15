use crate::script::lua_enter_point::SubScriptEntry;
use crate::script::Script;
use crate::script_manager::error::ScriptError;
use crate::UnityBundle;
use std::collections::BTreeMap;
use std::rc::Rc;

pub struct SubScriptManger<'lua> {
    pub(super) sub_scripts: Option<BTreeMap<String, Rc<SubScriptEntry<'lua>>>>,
    pub(super) script: &'lua Script<'static>,
}

pub struct SubScriptKeys<'lua, 'script> {
    keys: Option<std::collections::btree_map::Keys<'script, String, Rc<SubScriptEntry<'lua>>>>,
}

impl<'lua, 'script> Iterator for SubScriptKeys<'lua, 'script> {
    type Item = &'script str;

    fn next(&mut self) -> Option<Self::Item> {
        self.keys.as_mut()?.next().map(String::as_str)
    }
}

impl<'lua> SubScriptManger<'lua> {
    pub fn keys(&self) -> SubScriptKeys {
        SubScriptKeys {
            keys: self.sub_scripts.as_ref().map(|map| map.keys()),
        }
    }

    pub fn call_by_key(&self, key: &str, unity: UnityBundle) -> Result<(), ScriptError> {
        let scripts = self.sub_scripts.as_ref().ok_or_else(|| {
            ScriptError::SubScriptNotExist(format!("{}:{}", self.script.identity, key))
        })?;

        let sub_script = scripts.get(key).ok_or_else(|| {
            ScriptError::SubScriptNotExist(format!("{}:{}", self.script.identity, key))
        })?;

        sub_script.call(self.script.get_config(), unity, ())?;

        Ok(())
    }
}
