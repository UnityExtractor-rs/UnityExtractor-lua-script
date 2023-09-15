use std::{collections::BTreeMap, rc::Rc};

use mlua::{UserData, Value};

pub struct Log(Rc<str>);

impl Log {
    pub fn new(id: Rc<str>) -> Self {
        Self(id)
    }
}

impl UserData for Log {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("info", |_vm, this, args: BTreeMap<String, Value>| {
            println!("luaScript[{}] Info {}", this.0, map_to_str(args));
            Ok(())
        });
        methods.add_method("error", |_vm, this, args: BTreeMap<String, Value>| {
            println!("luaScript[{}] ERROR {}", this.0, map_to_str(args));
            Ok(())
        });
        methods.add_method("warn", |_vm, this, args: BTreeMap<String, Value>| {
            println!("luaScript[{}] WARN {}", this.0, map_to_str(args));
            Ok(())
        });
        methods.add_method("debug", |_vm, this, args: BTreeMap<String, Value>| {
            println!("luaScript[{}] DEBUG {}", this.0, map_to_str(args));
            Ok(())
        });
        methods.add_method("trace", |_vm, this, args: BTreeMap<String, Value>| {
            println!("luaScript[{}] TRACE {}", this.0, map_to_str(args));
            Ok(())
        });
    }
}

fn map_to_str(args: BTreeMap<String, Value>) -> String {
    args.into_iter()
        .map(|(k, v)| {
            format!(
                "`{k}` = {}",
                v.to_string().unwrap_or_else(|_| {
                    let ptr = v.to_pointer() as usize;
                    format!("Obj<0x{ptr:X}>")
                })
            )
        })
        .reduce(|l, r| format!("{l}, {r}"))
        .unwrap_or_default()
}
