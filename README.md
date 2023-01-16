# wasabi lang 

Simple language that compiles to WebAssembly.

## Examples

```go
import js {
  fn print(i32);
}

fn add(a: i32, b: i32): i32 {
  return a + b;
}

fn main() {
  print(1 + 2);
}
```
