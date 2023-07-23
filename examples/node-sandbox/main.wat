(module
  (func
    $main
    (export "main")
    (param $n i64)
    (result i64)
    (local $sum i64)
    (local $i i64)
    (local.set
      $sum
      (i64.extend_i32_s
        (i32.const 0)
      )
    )
    (local.set
      $i
      (i64.extend_i32_s
        (i32.const 0)
      )
    )
    (loop
      $wl0
      (block
        $wl1
        (i32.eqz
          (i64.lt_s
            (local.get $i)
            (local.get $n)
          )
        )
        (br_if $wl1)
        (local.set
          $sum
          (i64.add
            (local.get $sum)
            (local.get $i)
          )
        )
        (local.set
          $i
          (i64.add
            (local.get $i)
            (i64.extend_i32_s
              (i32.const 1)
            )
          )
        )
        (br $wl0)
      )
    )
    (local.get $sum)
  )
)
