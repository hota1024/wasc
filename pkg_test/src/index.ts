import { expectMain, failed, passed, testInstance } from './lib'

testInstance(
  `should export function that defined with "export"`,
  'export fn main(): i32 { return 1; }',
  (i) =>
    i.exports.main !== undefined
      ? passed()
      : failed(`'main' function not found`)
)
testInstance(
  `should not export function that defined without "export"`,
  'fn main(): i32 { return 1; }',
  (i) =>
    i.exports.main === undefined
      ? passed()
      : failed(`'main' function not found`)
)

expectMain(' 1 + 2      = 3', 'export fn main(): i32 { return 1 + 2; }', 3)
expectMain(' 1 + 2  * 3 = 7', 'export fn main(): i32 { return 1 + 2 * 3; }', 7)
expectMain(
  '(1 + 2) * 3 = 9',
  'export fn main(): i32 { return (1 + 2) * 3; }',
  9
)

expectMain(
  'last expr of block should be returned',
  'export fn main(): i32 { 2 }',
  2
)

testInstance(
  `params test`,
  'export fn add(a: i32, b: i32): i32 { return 1; }',
  (i) => {
    console.log(i.exports)
    return failed('nyarn')
  }
)
