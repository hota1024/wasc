import { compile } from '../../pkg/wasc'
import wabt from 'wabt'
import { white, bold, blue, red, bgGreen, bgRed } from 'colorette'

const getInstance = (source: string): Promise<WebAssembly.Instance> => {
  return new Promise((resolve, reject) => {
    wabt().then((wabt) => {
      const wat = compile(source)
      console.log(wat)
      const wasm = wabt.parseWat('main.wat', wat)
      const bin = wasm.toBinary({}).buffer

      WebAssembly.instantiate(bin)
        .then((i) => i.instance)
        .then(resolve)
    })
  })
}

export type Result =
  | {
      type: 'passed'
    }
  | {
      type: 'failed'
      message: string
    }

export const passed = (): Result => ({ type: 'passed' })

export const failed = (message: string): Result => ({ type: 'failed', message })

const report = (label: string, result: Result) => {
  if (result.type === 'passed') {
    console.log(`${bgGreen(white(' PASSED '))} ${bold(label)}`)
  } else {
    console.log(`${bgRed(white(' FAILED '))} ${bold(label)}`)
    console.log(`└─────── ${result.message}`)
  }
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
    report(label, passed())
  } else {
    report(
      label,
      failed(
        `expected: ${blue(JSON.stringify(expected))}, got: ${red(
          JSON.stringify(result)
        )}`
      )
    )
    // console.log(`${bgRed(' FAILED ')} ${bold(label)}`)
    // console.log(
    //   `└─────── expected: ${blue(JSON.stringify(expected))}, got: ${red(
    //     JSON.stringify(result)
    //   )}`
    // )
  }
}

export type TestInstanceCallback = (instance: WebAssembly.Instance) => Result

export const testInstance = async (
  label: string,
  source: string,
  fn: TestInstanceCallback
): Promise<void> => {
  const instance = await getInstance(source)
  report(label, fn(instance))
}
