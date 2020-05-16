import fs from 'fs'
import { MockService } from '@gtm/mock'

function readFile<T>(name: string): T {
  const bytes = fs.readFileSync(`assets/data/${name}.json`);
  return JSON.parse(bytes.toString());
}

async function writeJSON<T>(result: Promise<T>): Promise<void> {
  const json = JSON.stringify(await result)
  console.log(json)
}

(async (): Promise<void> => {
  const argv = process.argv

  if (argv.length <= 2) {
    console.log('No enough arguments provided for gtm-mock')
    process.exit(2)
  } else if (argv.length <= 2) {
    console.log('No enough arguments provided for gtm-mock export command')
    process.exit(3)
  } else {
    const command = argv[2]
    const service = await MockService.create(readFile)
    try {
      switch (command) {
        case 'commits':
          if (argv.length <= 4) {
            console.log('No enough arguments provided for data=commits')
            process.exit(5)
          }
          writeJSON(service.fetchCommits({ start: argv[3].split('=')[1], end: argv[4].split('=')[1] }))
          break
        case 'projects':
          writeJSON(service.fetchProjectList())
          break
        case 'status':
          writeJSON(service.fetchWorkdirStatus())
          break
        default:
          console.log('Unrecognized gtm-mock export sub-command, got:', argv)
          process.exit(6)
      }
    } catch (err) {
      console.log(err)
      process.exit(7)
    }
  }

})()