import { test } from "./test.js"


console.log("from dev.js")

const a = test()
console.log(a)

document.body.appendChild(a)