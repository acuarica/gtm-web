import { MockService } from '@gtm/mock'

async function writeJSON<T>(result: Promise<T>): Promise<void> {
  const json = JSON.stringify(await result)
  console.log(json)
}

(async (): Promise<void> => {
  const argv = process.argv

  if (argv.length <= 2) {
    console.log('No enough arguments provided for gtm-mock')
    process.exit(2)
  } else if (argv[2] !== 'export') {
    console.log('Command argument for gtm-mock must be `export`')
    process.exit(3)
  } else if (argv.length <= 3) {
    console.log('No enough arguments provided for gtm-mock export command')
    process.exit(4)
  } else {
    const command = argv[3]
    const service = new MockService()
    try {
      switch (command) {
        case '-data=commits':
          if (argv.length <= 5) {
            console.log('No enough arguments provided for data=commits')
            process.exit(5)
          }
          writeJSON(service.fetchCommits({ start: argv[4].split('=')[1], end: argv[5].split('=')[1] }))
          break
        case '-data=projects':
          writeJSON(service.fetchProjectList())
          break
        case '-data=status':
          writeJSON(service.fetchWorkdirStatus())
          break
        default:
          console.log('Unrecognized gtm-mock export sub-command')
          process.exit(6)
      }
    } catch (err) {
      console.log(err)
      process.exit(7)
    }
  }

})()