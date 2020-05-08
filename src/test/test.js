
import Test from "./Test.svelte";

console.log("from test this time")

new Test({
  target: document.body
})


export function test() {
  const d = document.createElement('div')
  d.innerHTML = "<a href='asdf'>hola</a>"
  return d
  return "exporting from module"
}