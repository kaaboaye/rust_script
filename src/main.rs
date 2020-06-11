extern crate rusty_v8;

use rusty_v8 as v8;
use std::io::{self, BufRead};

fn main() {
    println!("Hello, world!");

    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let mut isolate = v8::Isolate::new(Default::default());

    let mut handle_scope = v8::HandleScope::new(&mut isolate);
    let scope = handle_scope.enter();

    let context = v8::Context::new(scope);
    let mut context_scope = v8::ContextScope::new(scope, context);
    let scope = context_scope.enter();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let code = v8::String::new(scope, line.as_str()).unwrap();

        let mut script = v8::Script::compile(scope, context, code, None).unwrap();
        let result = script.run(scope, context).unwrap();
        let result = result.to_string(scope).unwrap();
        println!("{}", result.to_rust_string_lossy(scope));
    }
}
