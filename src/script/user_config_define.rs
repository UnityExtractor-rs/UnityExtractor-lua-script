use crate::script::storage::Storage;
use crate::script::user_editable_config::ConfigKind;
use crate::script_libs::config_manage::{UserEditableConfig, UserEditableConfigKind};
use crate::script_libs::error::InitScriptError;
use std::collections::BTreeMap;
use std::rc::Rc;

use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct UserConfigDefine {
    #[builder(setter(transform = |identity:&str| Rc::from(String::from(identity).into_boxed_str())))]
    script_identity: Rc<str>,

    defines: BTreeMap<Rc<str>, UserEditableConfig>,
}

impl UserConfigDefine {
    pub fn update(
        &self,
        storage: &impl Storage,
        identity: &str,
        value: ConfigKind,
    ) -> Result<Option<String>, InitScriptError> {
        if let Some(define) = self.defines.get(identity) {
            let v = match (&*define.kind, value) {
                (UserEditableConfigKind::Select(selects, ..), ConfigKind::Select(k)) => {
                    if !selects.contains_key(&k) {
                        return Err(InitScriptError::SelectTargetNotInRage(
                            k,
                            selects.keys().cloned().collect(),
                        ));
                    }
                    k
                }
                (UserEditableConfigKind::Switch(_), ConfigKind::Switch(bo)) => bo.to_string(),
                (UserEditableConfigKind::Text(_), ConfigKind::Text(s)) => s,
                (l, r) => return Err(InitScriptError::UserConfigTypeNotMatch(l.ty(), r.ty())),
            };

            storage
                .store(&self.script_identity, identity, v.as_bytes())
                .map_err(InitScriptError::Storage)?;
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }
}
