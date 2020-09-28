import * as React from 'react'
import { render } from 'react-dom'
// App wrapped with redux Provider, store, etc.

import { Home } from '~/components/Home'

render(
  <Home />,
  document.getElementById('root')
)
