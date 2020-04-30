
import { fetchCommits } from '.'

(async (): Promise<void> => {
  
  const range = {start: '2020-04-01', end:'2020-04-30'}
  const c = await fetchCommits(range)
  console.log(c)
})()