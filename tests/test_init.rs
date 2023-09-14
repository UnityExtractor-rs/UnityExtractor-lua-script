use extactor_lua_embedding::{
    script::storage::{BoxedStorage, StorageResult},
    script_libs::ScriptRegister,
    UnityBundle,
};
use mlua::{Function, Lua};

use std::{cell::RefCell, collections::HashMap, error::Error, path::Path, rc::Rc};

#[test]
fn test() {
    let script = Path::new("test_lua/test_init.lua");
    let storage = Rc::new(MapStorage(RefCell::new(HashMap::new()))) as BoxedStorage;
    let lua = Lua::new();
    lua.load(script).exec().unwrap();

    let init: Function = lua.globals().get("InitScript").unwrap();

    // init script
    let ret: ScriptRegister = init.call(()).unwrap();

    println!("{:?}", ret);

    // script and config define
    let script = ret.to_script(&storage);
    let define = ret.to_config_define();

    println!("{:#?}", script);
    // start config update
    script.configs.iter().for_each(|(k, v)| {
        define.update(&script.storage, &k, v.kind.clone()).unwrap();
    });
    script
        .entry_point
        .update_config(&lua, script.configs.iter().map(|(k, v)| (&**k, v.value())))
        .unwrap();

    // user select a unity bundle
    let map = script
        .entry_point
        .get_matched_scripts(UnityBundle::default())
        .unwrap()
        .unwrap();
    println!("{map:?}");

    // user using function "A"
    let entry = map.get("A").unwrap();

    // run function A
    let _: () = entry
        .call(script.get_config(), UnityBundle::default(), ())
        .unwrap();
}

#[derive(Debug)]
struct MapStorage(RefCell<HashMap<String, Box<[u8]>>>);

impl extactor_lua_embedding::script::storage::Storage for MapStorage {
    fn load(&self, script: &str, key: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        println!("get{script}.{key}",);
        Ok(self
            .0
            .borrow()
            .get(&format!("{script}.{key}"))
            .map(|v| v.clone().into_vec())
            .unwrap_or_default())
    }

    fn store(&self, script: &str, key: &str, value: &[u8]) -> StorageResult<()> {
        println!(
            "save {script}.{key}, value: {}",
            String::from_utf8_lossy(value)
        );
        self.0
            .borrow_mut()
            .insert(format!("{script}.{key}"), value.to_vec().into_boxed_slice());
        Ok(())
    }
}
