use mlua::{IntoLua, Lua, UserData, UserDataFields, Value};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum MetaValue {
    Str(Rc<str>),
    Number(i64),
    Float(f64),
    Bool(bool),
}

impl MetaValue {
    pub fn ty(&self) -> &'static str {
        match self {
            MetaValue::Str(_) => "string",
            MetaValue::Number(_) => "number",
            MetaValue::Float(_) => "float",
            MetaValue::Bool(_) => "bool",
        }
    }
    pub fn to_value<'lua>(&self, vm: &'lua Lua) -> mlua::Result<Value<'lua>> {
        match self {
            MetaValue::Str(s) => s.into_lua(vm),
            MetaValue::Number(n) => n.into_lua(vm),
            MetaValue::Float(n) => n.into_lua(vm),
            MetaValue::Bool(b) => Ok(Value::Boolean(*b)),
        }
    }
}

impl UserData for MetaValue {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("ty", |_vm, this| Ok(this.ty()));
        fields.add_field_method_get("value", |vm, this| this.to_value(vm));
    }
}
