import fs from 'fs'

const [commits, projects, workdir] = ['commits', 'projects', 'workdir']
  .map(name => {
    const bytes = fs.readFileSync(`@mock/data/data-${name}.json`);
    return JSON.parse(bytes);
  })

export { commits, projects, workdir }