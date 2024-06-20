import test from 'ava'

import { startListener } from '../index'

test('sync function from native code', (t) => {
  t.pass()
  startListener((data) => {
    const dd = JSON.parse(data)
    dd.data = JSON.parse(dd.data)
  })
})
