
// import CS from './Commit-test';

console.log('from test this time')

export function test(): Element {
  const d = document.createElement('div')
  d.innerHTML = '<a href=\'asdf\'>hola</a>'
  document.body.appendChild(d)
  return d
}