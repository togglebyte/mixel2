use std::cell::RefCell;
use std::env::{set_current_dir, current_dir};
use std::fs::{read, read_dir, DirEntry};
use std::path::{Component, Path};

use mlua::prelude::*;
use mlua::{Lua, Result, Function, Variadic};
use nightmaregl::Position;
use nightmaregl::pixels::Pixel;

use crate::canvas::Containers;

#[derive(Debug)]
pub enum Arg {
    String(String),
    Number(f64),
    Bool(bool),
}

impl Arg {
    pub fn from_str(s: String) -> Option<Arg> {
        if s == "false" {
            return Some(Arg::Bool(false));
        }

        if s == "true" {
            return Some(Arg::Bool(true));
        }

        let arg = match s.parse::<f64>() {
            Ok(num) => Arg::Number(num),
            Err(_) => Arg::String(s),
        };

        Some(arg)
    }
}

// -----------------------------------------------------------------------------
//     - Plugin call -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct PluginCall {
    name: String,
    args: Vec<Arg>,
}

impl PluginCall {
    pub fn new(name: String, args: Vec<Arg>) -> Self {
        Self {
            name,
            args,
        }
    }
}

// -----------------------------------------------------------------------------
//     - Plugin -
// -----------------------------------------------------------------------------
fn load_plugin(lua: &Lua, path: &Path) -> Result<()> {
    let cwd = current_dir().unwrap();

    let name = match path.components().last() {
        Some(Component::Normal(name)) => name.to_str().unwrap(),
        _ => return Ok(()), // TODO: meh, fix this you lazy sausage
    };

    let path = path.join("autoload.lua");
    let plugin_src = read(path)?;

    let new_path = cwd.join("plugins").join(name);
    set_current_dir(new_path);
    match lua.load(&plugin_src).exec() {
        Ok(_) => eprintln!("Loaded: {}", name),
        Err(e) => eprintln!("Loading {} failed: {:?}", name, e),
    }
    set_current_dir(cwd);

    Ok(())
}

pub struct Plugin {
    lua: Lua,
}

impl Plugin {
    pub fn new() -> Result<Self> {
        let lua = Lua::new();

        // TODO: It's obvious that we shouldn't do this:
        //       Plugins should be read from XDG_CONFIG/mixel/plugins
        //       and make sure the dir exists (create?)
        let plugins = read_dir("plugins")?;
        for entry in plugins {
            let path = entry?.path();
            if path.is_dir() {
                load_plugin(&lua, &path);
            }
        }

        let inst = Self {
            lua
        };

        Ok(inst)
    }

    // TODO add app context, that contains viewport, app things
    pub fn exec_code(&mut self, code: &str, containers: &mut Containers) -> LuaResult<()> {
        let containers = RefCell::new(containers);
        self.lua.scope(|scope| {
            let globals = self.lua.globals();

            let f = scope.create_function_mut(|_, (x, y): (i32, i32)| {
                let mut containers = containers.borrow_mut();
                containers.draw(Position::new(x, y));
                Ok(())
            }).unwrap();
            globals.set("putPixel", f);

            let f = scope.create_function_mut(|_, (r, g, b): (u8, u8, u8)| {
                let mut containers = containers.borrow_mut();
                let pixel = Pixel { r, g, b, ..Default::default() };
                containers.set_colour(pixel);
                Ok(())
            }).unwrap();
            globals.set("setColor", f);

            match self.lua.load(code).exec() {
                Ok(_) => {}
                Err(e) =>  eprintln!("Lua err: {:?}", e),
            }
            Ok(())
        });

        Ok(())
    }

    // pub fn exec(&mut self, call: &PluginCall) -> LuaResult<()> {
    //     let PluginCall { name, args } = call;
    //     let globals = self.lua.globals();

    //     let parts = call.name.split('.');
    //     let f: Function = globals.raw_get(&call.name as &str)?;
    //     eprintln!("{:?}", call.name);
    //     let args = call.args.iter().map(|a| to_lua(&self.lua, a)).collect::<Variadic<_>>();
    //     f.call::<_, ()>(args);
    //     Ok(())
    // }
}

fn to_lua<'a>(lua: &'a Lua, arg: &'a Arg) -> LuaValue<'a> {
    match arg {
        Arg::String(s) => s.clone().to_lua(lua).unwrap(),
        Arg::Bool(b) => LuaValue::Boolean(*b),
        Arg::Number(n) => LuaValue::Number(*n)
    }
}
