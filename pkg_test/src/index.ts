import { expectMain } from './lib'

expectMain(' 1 + 2      = 3', 'fn main(): i32 { 1 + 2; }', 3)
expectMain(' 1 + 2  * 3 = 7', 'fn main(): i32 { 1 + 2 * 3; }', 7)
expectMain('(1 + 2) * 3 = 9', 'fn main(): i32 { (1 + 2) * 3; }', 9)
