import process from 'node:process'
import rdev from '@rdev-js/core'

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

async function main() {
  rdev.mouseMove(700, 900)
  rdev.buttonPress(rdev.Button.Left)
  await sleep(500)
  rdev.buttonRelease(rdev.Button.Left)
  await sleep(1000)
  rdev.keyPress(rdev.Key.KeyA)
  await sleep(500)
  rdev.keyPress(rdev.Key.KeyA)
}

main()

function watch(direction, callback) {
  rdev.listen((event) => {
    if (event.type.direction === direction)
      callback(event)
  })

  return () => process.exit(0)
}

const unwatch = watch('KeyPress', (event) => {
  // console.log(event)

  if (event.type.key === 'KeyQ')
    unwatch()
})
