// src/vm/mod.rs

use wasmtime::*;
use log::info;

pub struct VM {
    engine: Engine,
    linker: Linker<()>,
}

impl VM {
    pub fn new() -> Self {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);

        // 호스트 함수 정의
        linker.func_wrap("env", "log", |msg_ptr: i32, msg_len: i32| {
            // 메시지를 메모리에서 읽어와 로그 출력
            // 메모리 접근을 위한 추가 코드 필요
            // ...
        }).unwrap();

        VM { engine, linker }
    }

    pub fn execute(&mut self, wasm_bytes: &[u8]) {
        let module = Module::from_binary(&self.engine, wasm_bytes).unwrap();
        let mut store = Store::new(&self.engine, ());
        let instance = self.linker.instantiate(&mut store, &module).unwrap();

        // '_start' 함수 호출
        let start = instance.get_typed_func::<(), (), _>(&mut store, "_start").unwrap();
        start.call(&mut store, ()).unwrap();
    }
}
