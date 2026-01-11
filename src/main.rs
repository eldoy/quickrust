use rquickjs::{Context, Runtime, function::Func, Object};
use std::fs;

fn main() {
    let rt = Runtime::new().unwrap();
    let ctx = Context::full(&rt).unwrap();

    ctx.with(|ctx| -> Result<(), rquickjs::Error> {
        let global = ctx.globals();

        global.set("hello", Func::from(|| "from Rust"))?;

        let console = Object::new(ctx.clone())?;
        console.set("log", Func::from(|s: String| {
            println!("{}", s);
        }))?;
        global.set("console", console)?;

        ctx.eval::<(), _>(r#"
            function add(a, b) {
                return a + b
            }
        "#)?;

        let code = fs::read_to_string("script.js")?;
        ctx.eval::<(), _>(code)?;

        Ok(())
    }).unwrap();
}
