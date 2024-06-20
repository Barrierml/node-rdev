import test from 'ava'

import { startListener } from '../index'

test('sync function from native code', (t) => {
  t.pass()
  console.log(1233333)
  startListener((data) => {
    const dd = JSON.parse(data)
    dd.data = JSON.parse(dd.data)
    console.log('123', dd)
  })
  console.log(1232222)
})
