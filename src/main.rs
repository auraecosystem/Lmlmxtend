#![allow(unused)]
fn main() {
use lmlmxtend_api::CapabilityId;
use lmlmxtend_intelligence::{IntelligenceRouter, TaskIntent};
use lmlmxtend_runtime::Runtime;

let router = IntelligenceRouter;
let intent = TaskIntent::new(
    CapabilityId::new("creative.compose")?,
    "Create a scene outline",
);
let plan = router.plan(&runtime, &intent)?;
let response = router.execute(&runtime, intent)?;
}
