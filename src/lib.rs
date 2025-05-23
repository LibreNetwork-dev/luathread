use mlua::{Lua, Result, Function, Value, Table, lua_module};
use std::thread;

#[lua_module]
fn luathread(lua: &Lua) -> Result<Table> {
    let exports = lua.create_table()?;
    let spawn = lua.create_function(|_, func: Function| {
        let dumped = func.dump(true);
                // rust compiler I need move, so I put move, anyone who acc knows rust kill me for it pls
        thread::spawn(move || {
            let l_thread = unsafe {Lua::new()}; 

            let func = l_thread.load(&dumped).into_function()
                .expect("failed to load function");
            func.call::<_, Value>(()).expect("call failed");
        });

        Ok(())
    })?;

    exports.set("spawn", spawn)?;
    Ok(exports)
}
