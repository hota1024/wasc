(module
  (import
    "js"
    "print"
    (func
      $print
      (param i32)
    )
  )
  (func
    $fib
    (param $n i32)
    (result i32)
    (if
      (i32.le_s
        (local.get $n)
        (i32.const 1)
      )
      (then
        (return
          (local.get $n)
        )
      )
    )
    (return
      (i32.add
        (call
          $fib
          (i32.sub
            (local.get $n)
            (i32.const 1)
          )
        )
        (call
          $fib
          (i32.sub
            (local.get $n)
            (i32.const 2)
          )
        )
      )
    )
  )
  (func
    $_start
    (export "_start")
    (call
      $print
      (call
        $fib
        (i32.const 10)
      )
    )
  )
)
