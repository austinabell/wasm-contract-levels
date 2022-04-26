(module
  (import "env" "log_utf8" (func $env.log (param i64 i64)))
  (func (export "hello")
    i64.const 11
    i64.const 0
    call $env.log
  )
  (memory $memory 11)
  (export "memory" (memory 0))
  (data $d0 (i32.const 0) "Hello World")
)
