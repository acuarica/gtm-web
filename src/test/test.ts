import '../../main.pcss'
import CS from './Commit-test';

console.log('from test this time')

export function test(): Element {
  const d = document.createElement('div')
  d.innerHTML = '<a href=\'asdf\'>holsdfias</a>'
  document.body.appendChild(d)



  new CS.component({
    target: d,
    props: CS.props,
  })

  return d
}

test()