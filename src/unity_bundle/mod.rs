use mlua::UserData;
use std::marker::PhantomData;

#[derive(Default)]
pub struct UnityBundle {
    __phantom: PhantomData<()>,
}

impl UserData for UnityBundle {}
