use crate::script::storage::Storage;
use crate::script::user_editable_config::ConfigValue;
use crate::script_loader::config::{UserEditableConfig, UserEditableConfigKind};
use crate::script_loader::error::UserConfigError;
use std::collections::BTreeMap;
use std::rc::Rc;

use typed_builder::TypedBuilder;

use super::error::{ScriptInnerError, StorageError};

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
        value: ConfigValue,
    ) -> Result<String, ScriptInnerError> {
        let Some(define) = self.defines.get(identity) else {
            return Err(UserConfigError::NotDefine {
                id: identity.to_string(),
            }
            .into());
        };
        let v = match (&*define.kind, value) {
            (UserEditableConfigKind::Select(selects, ..), ConfigValue::Select(k)) => {
                if !selects.contains_key(&k) {
                    return UserConfigError::new_out_range(k, selects.keys());
                }
                k
            }
            (UserEditableConfigKind::Switch(_), ConfigValue::Switch(bo)) => bo.to_string(),
            (UserEditableConfigKind::Text(_), ConfigValue::Text(s)) => s,
            (l, r) => return Err(UserConfigError::new_not_match(l.ty(), r.ty()).into()),
        };

        storage
            .store(&self.script_identity, identity, v.as_bytes())
            .map_err(StorageError::from)?;
        Ok(v)
    }
}
