import { compile } from '../../pkg/wasc'
import wabt from 'wabt'
import { bold, blue, red, bgGreen, bgRed } from 'colorette'

const getInstance = (source: string): Promise<WebAssembly.Instance> => {
  return new Promise((resolve, reject) => {
    wabt().then((wabt) => {
      const wat = compile(source)
      const wasm = wabt.parseWat('main.wat', wat)
      const bin = wasm.toBinary({}).buffer

      WebAssembly.instantiate(bin)
        .then((i) => i.instance)
        .then(resolve)
    })
  })
}

export const expectMain = async (
  label: string,
  source: string,
  expected: any
): Promise<void> => {
  const instance = await getInstance(source)
  const main = instance.exports.main as () => any
  const result = main()

  if (result === expected) {
    console.log(`${bgGreen(' PASSED ')} ${bold(label)}`)
  } else {
    console.log(`${bgRed(' FAILED ')} ${bold(label)}`)
    console.log(
      `└─────── expected: ${blue(JSON.stringify(expected))}, got: ${red(
        JSON.stringify(result)
      )}`
    )
  }
}
