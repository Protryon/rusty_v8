// Copyright 2019-2020 the Deno authors. All rights reserved. MIT license.
use rusty_v8_protryon as v8;

pub fn main() {
  let mut isolate = v8::Isolate::new(mock());
  let mut hs1 = v8::HandleScope::new(&mut isolate);
  let hs1 = hs1.enter();

  let _local = {
    let mut hs2 = v8::HandleScope::new(hs1);
    let hs2 = hs2.enter();

    let mut hs3 = v8::EscapableHandleScope::new(hs2);
    let hs3 = hs3.enter();

    let value: v8::Local<v8::Value> = v8::Integer::new(hs3, 42).into();
    hs3.escape(value)
  };
}

fn mock<T>() -> T {
  unimplemented!()
}
