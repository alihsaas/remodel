use core::fmt;

use anyhow::format_err;
use rlua::{UserData, Context, MetaMethod, UserDataMethods, ToLua};
use crate::value::EnumValue;

#[derive(Debug, Clone)]
pub struct EnumItemUserData(String);

impl EnumItemUserData {
    fn meta_index<'lua>(
        &self,
        context: Context<'lua>,
        key: &str,
    ) -> rlua::Result<rlua::Value<'lua>> {
        let database = rbx_reflection_database::get();

        let enum_descriptor = database.enums.get(self.0.as_str()).ok_or_else(|| {
            rlua::Error::external(format_err!("Unknown enum {}. This is a Rojo bug!", self.0))
        })?;

        let item = enum_descriptor.items.get(key).ok_or_else(|| {
            rlua::Error::external(format_err!("Unknown enum item {}. This is a Rojo bug!", key))
        })?;

        EnumValue::from(item).to_lua(context)
    }
}

impl fmt::Display for EnumItemUserData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Enum.{}", &self.0)
    }
}

impl UserData for EnumItemUserData {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::Eq, |context, this, rhs: Self| {
            (this.0 == rhs.0).to_lua(context)
        });

        methods.add_meta_method(MetaMethod::Index, |context, this, key: String| {
            this.meta_index(context, &key)
        }); 

        methods.add_meta_method(MetaMethod::ToString, |context, this, _arg: ()| {
            this.to_string().to_lua(context)
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnumUserData;

impl EnumUserData {
    fn meta_index<'lua>(
        &self,
        _context: Context<'lua>,
        key: &str,
    ) -> rlua::Result<EnumItemUserData> {
        let database = rbx_reflection_database::get();

        database.enums.get(key).ok_or_else(|| {
            rlua::Error::external(format_err!("Unknown enum {}. This is a Rojo bug!", key))
        })?;

        Ok(EnumItemUserData(key.to_string()))
    }
}

impl UserData for EnumUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |context, this, key: String| {
            this.meta_index(context, &key)
        });
    }
}