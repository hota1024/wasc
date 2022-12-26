(module
  (func
    $add
    (param $a i32)
    (param $b i32)
    (drop
      (i32.add
        (local.get $a)
        (local.get $b)
      )
    )
  )
  (func
    $main
    (export "main")
    (call
      $add
      (i32.const 10)
      (i32.const 20)
    )
  )
)
