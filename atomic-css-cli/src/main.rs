// TODO: experiment with rune instead of a custom language
//
// use rune::termcolor::{ColorChoice, StandardStream};
// use rune::{Context, Diagnostics, Source, Sources, Vm};
// use std::sync::Arc;
//
// fn main() -> rune::support::Result<()> {
//     let context = Context::with_default_modules()?;
//     let runtime = Arc::new(context.runtime()?);
//
//     let mut sources = Sources::new();
//     sources.insert(Source::from_path("./test.rn")?)?;
//
//     let mut diagnostics = Diagnostics::new();
//
//     let result = rune::prepare(&mut sources)
//         .with_context(&context)
//         .with_diagnostics(&mut diagnostics)
//         .build();
//
//     if !diagnostics.is_empty() {
//         let mut writer = StandardStream::stderr(ColorChoice::Always);
//         diagnostics.emit(&mut writer, &sources)?;
//     }
//
//     let unit = result?;
//     let mut vm = Vm::new(runtime, Arc::new(unit));
//     vm.call(["main"], ())?;
//
//     Ok(())
// }
//

use atomic_css_runtime::Runtime;
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("Expected file argument");
    let src = fs::read_to_string(filename).expect("Failed to read file");

    let mut runtime = Runtime::new();
    runtime.run(&src);

    println!("{:?}", &runtime.organism);
}
