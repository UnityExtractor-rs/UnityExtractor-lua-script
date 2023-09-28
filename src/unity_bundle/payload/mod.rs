mod text_payload;

use mlua::UserData;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Payload {
    Image(),
    Binary(),
    Text(),
    Mesh(),
}

impl UserData for Payload {}
