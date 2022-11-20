import { testInstance, passed, failed } from './lib'

testInstance(
  `should export a function that defined with "export" and returns 0;`,
  `export fn main(): i32 { 1; }`,
  (i) =>
    i.exports.main !== undefined
      ? passed()
      : failed(`'main' function is not defined`)
)
