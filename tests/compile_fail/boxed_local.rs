// Copyright 2019-2020 the Deno authors. All rights reserved. MIT license.
use rusty_v8_protryon as v8;

pub fn main() {
  let mut isolate = v8::Isolate::new(mock());

  let _boxed_local = {
    let mut hs = v8::HandleScope::new(&mut isolate);
    let hs = hs.enter();
    Box::new(v8::Integer::new(hs, 123))
  };
}

fn mock<T>() -> T {
  unimplemented!()
}
