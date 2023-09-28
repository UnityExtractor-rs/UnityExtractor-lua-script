use mlua::UserData;
use std::marker::PhantomData;

pub mod unity_object;
mod meta_value;
mod payload;

#[derive(Default)]
pub struct UnityBundle {
    __phantom: PhantomData<()>,
}

impl UserData for UnityBundle {}
