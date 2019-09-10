use netbricks::common::Result;
use netbricks::config::load_config;
use netbricks::runtime::Runtime;

mod simple;

fn main() -> Result<()> {
    let configuration = load_config()?;
    println!("config: {}", configuration);
    let runtime = Runtime::init(&configuration)?;

    let mut runtime = simple::run(runtime);

    runtime.execute()?;

    Ok(())
}
