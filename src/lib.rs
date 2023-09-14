mod script_libs;
mod unity_object;

use mlua::Lua;

fn lua_loader() {
    let vm = Lua::new();
}

pub use script_libs::script_register::ScriptRegister;
