import {
  mouseMove,
  buttonPress,
  buttonRelease,
  Button,
  Key,
  keyPress,
  callThreadsafeFunction
} from '@rdev-js/core'

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

// async function main () {
//   mouseMove(700,900)
//   buttonPress(Button.Left)
//   await sleep(500)
//   buttonRelease(Button.Left)
//   await sleep(1000)
//   keyPress(Key.KeyA)
//   await sleep(500)
//   keyPress(Key.KeyA)
// }

// main()
