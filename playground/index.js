import { Button, buttonPress, buttonRelease, displaySize, Key, keyPress, listen, mouseMove } from 'rdev-js'

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

function moveMouseSmoothly(startX, startY, endX, endY, steps, delay) {
  const deltaX = (endX - startX) / steps
  const deltaY = (endY - startY) / steps

  let currentX = startX
  let currentY = startY

  function moveStep(step) {
    if (step > steps)
      return

    currentX += deltaX
    currentY += deltaY
    mouseMove(currentX, currentY)

    setTimeout(() => moveStep(step + 1), delay)
  }

  moveStep(1)
}

async function main() {
  moveMouseSmoothly(1000, 1000, 600, 700, 100, 5)
  buttonPress(Button.Left)
  await sleep(500)
  buttonRelease(Button.Left)
  await sleep(1000)
  keyPress(Key.KeyH)
  keyPress(Key.KeyE)
  keyPress(Key.KeyL)
  await sleep(50)
  keyPress(Key.KeyL)
  keyPress(Key.KeyO)
  keyPress(Key.Space)
  await sleep(500)
  keyPress(Key.KeyW)
  keyPress(Key.KeyO)
  keyPress(Key.KeyR)
  keyPress(Key.KeyL)
  keyPress(Key.KeyD)
  await sleep(500)
  for (let i = 0; i < 5; i++) {
    keyPress(Key.Backspace)
    await sleep(100)
  }
  keyPress(Key.KeyR)
  keyPress(Key.KeyD)
  keyPress(Key.KeyE)
  keyPress(Key.KeyV)
  await sleep(200)
  // 按下-键
  keyPress(Key.Minus)
  keyPress(Key.KeyJ)
  keyPress(Key.KeyS)
}

main()
// console.log('hello world')

// listen((e) => {
//   console.log(e)
// })
