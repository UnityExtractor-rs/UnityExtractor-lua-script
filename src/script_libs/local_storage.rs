use mlua::{UserData, UserDataMethods};

pub struct LocalStorage {
    plugin_name: String,
}

impl LocalStorage {
    fn new(plugin_name: &str) -> Self {
        todo!()
    }
    fn load_config(&self, key: &str) -> Option<String> {
        todo!()
    }
    fn save_config(&self, key: &str, value: impl ToString) {
        todo!()
    }
}

impl UserData for LocalStorage {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // methods.add_function();
        methods.add_method("load", |this, v, arg: String| Ok("LoadedValue".to_string()));
    }
}
