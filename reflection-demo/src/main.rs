use serde::{Serialize, Deserialize};

#[typetag::serde]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn execute(&self);
}

#[derive(Serialize, Deserialize)]
struct HelloPlugin;
#[derive(Serialize, Deserialize)]
struct ByePlugin;

#[typetag::serde]
impl Plugin for HelloPlugin {
    fn name(&self) -> &'static str { "HelloPlugin" }
    fn execute(&self) { println!("Hello from plugin!"); }
}

#[typetag::serde]
impl Plugin for ByePlugin {
    fn name(&self) -> &'static str { "ByePlugin" }
    fn execute(&self) { println!("Goodbye!"); }
}

inventory::collect!(&'static dyn Plugin);

inventory::submit! {
    &HelloPlugin as &'static dyn Plugin
}

inventory::submit! {
    &ByePlugin as &'static dyn Plugin
}

fn main() {
    for plugin in inventory::iter::<&'static dyn Plugin> {
        println!("Running {:?}", plugin.name());
        plugin.execute();
    }
}