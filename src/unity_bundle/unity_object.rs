use crate::unity_bundle::meta_value::MetaValue;
use crate::unity_bundle::payload::Payload;
use mlua::{IntoLua, UserData, UserDataFields};
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

pub struct UnityObject {
    identity: u64,
    name: Rc<str>,
    meta: Rc<BTreeMap<String, MetaValue>>,
    payload: Rc<Payload>,
}

impl UserData for UnityObject {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("identity", |vm, this| Ok(this.identity));
        fields.add_field_method_get("name", |vm, this| this.name.into_lua(vm));
        fields.add_field_method_get("meta", |vm, this| {
            let table =
                vm.create_table_from(this.meta.iter().map(|(k, v)| (k.as_str(), v.clone())))?;
            Ok(table)
        });
        fields.add_field_method_get("payload", |vm, this| Ok(this.payload.clone()))
    }
}
