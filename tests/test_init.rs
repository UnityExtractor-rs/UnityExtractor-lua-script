use extactor_lua_embedding::{
    script::storage::{BoxedStorage, StorageResult},
    script_libs::ScriptRegister,
    UnityBundle,
};
use mlua::{Function, Lua, LuaOptions};

use extactor_lua_embedding::script_manager::ScriptManager;
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
#[test]
fn wrap_init() {
    let script = Path::new("test_lua/test_init.lua");
    let storage = Rc::new(MapStorage(RefCell::new(HashMap::new()))) as BoxedStorage;
    let mut script_manager = ScriptManager::new(LuaOptions::new(), storage).unwrap();

    // load script
    println!("load script");
    script_manager.load_entry(&script).unwrap();
    // init script
    println!("init script");
    script_manager.init_script().unwrap();

    //init done

    // user select item
    let scripts = script_manager
        .get_match_scripts(UnityBundle::default())
        .unwrap();

    for key in scripts.keys() {
        println!("match key:{key}")
    }

    // user press key A
    scripts.call_by_key("A", UnityBundle::default()).unwrap();
    scripts.call_by_key("B", UnityBundle::default()).unwrap();

    println!("done")
}

#[derive(Debug)]
struct MapStorage(RefCell<HashMap<String, Box<[u8]>>>);

impl extactor_lua_embedding::script::storage::Storage for MapStorage {
    fn contains_key(&self, script: &str, key: &str) -> StorageResult<bool> {
        Ok(self.0.borrow().contains_key(&format!("{script}.{key}")))
    }

    fn load(
        &self,
        script: &str,
        key: &str,
    ) -> Result<Option<Vec<u8>>, Box<dyn Error + Send + Sync>> {
        println!("get{script}.{key}",);
        Ok(self
            .0
            .borrow()
            .get(&format!("{script}.{key}"))
            .map(|v| v.clone().into_vec()))
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
