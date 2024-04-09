import { describe, expect, it } from 'vitest'

import {
  Button,
  buttonPress,
  buttonRelease,
  listen,
} from '../index.js'

function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

describe('button', () => {
  let buttonEvent
  listen((event) => {
    buttonEvent = event.type
  })
  it('button press', async () => {
    await sleep(20)
    buttonPress(Button.Left)
    await sleep(20)
    expect(buttonEvent).toEqual({
      direction: 'ButtonPress',
      key: 'Left',
    })
  })

  it('button release', async () => {
    await sleep(20)
    buttonRelease(Button.Left)
    await sleep(20)
    expect(buttonEvent).toEqual({
      direction: 'ButtonRelease',
      key: 'Left',
    })
  })
})
