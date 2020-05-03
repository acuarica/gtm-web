

const fs = require('fs')
const js = require('./src/data-commits.json')

const cs = []
for (const c of js) {
  if (c.Note && c.Note.Files.length > 0) {
    cs.push(c)
    if (cs.length === 50) break
  }
}

console.log(js.length)
console.log(cs.length)

// stringify JSON Object
var jsonContent = JSON.stringify(cs);
// console.log(jsonContent);

fs.writeFile("output.json", jsonContent, () => { })