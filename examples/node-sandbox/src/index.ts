import * as fs from 'fs/promises'
import * as npath from 'path'
import wabt from 'wabt'

const wat2wasm = (wat: string): Promise<Buffer> => {
  return new Promise((resolve, reject) => {
    wabt().then(function(wabt: any) {
      const wasm = wabt.parseWat('main.wat', wat)
      const bin = wasm.toBinary({}).buffer

      resolve(bin)
    })
  })
}

const main = async () => {
  const wat = await fs
    .readFile(npath.join(process.cwd(), 'main.wat'))
    .then((v) => v.toString())
  const buffer = await wat2wasm(wat)
  const { instance } = (await WebAssembly.instantiate(buffer, {
    std: {
      printNum(v: number) {
        console.log(v)
      },
    },
  })) as any
  instance.exports.main()
}

main()
