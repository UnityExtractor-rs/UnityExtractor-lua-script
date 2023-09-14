use extactor_lua_embedding::ScriptRegister;
use mlua::{Function, Lua};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

#[test]
fn test() {
    let script = Path::new("tests/test_init.lua");

    let lua = Lua::new();
    lua.load(script).exec().unwrap();

    let init: Function = lua.globals().get("initScript").unwrap();

    let register = Rc::new(RefCell::new(ScriptRegister::default()));
    let _: () = init.call(Rc::clone(&register)).expect("Err");

    println!("{:#?}", register.borrow());
}
