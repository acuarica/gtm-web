import sirv from 'sirv';
import { fetchCommits, fetchProjectList } from './src/git';
import polka from 'polka';
import send from '@polka/send-type';

export function startServe(dir, port) {

  const assets = sirv(dir, {
    maxAge: 31536000, // 1Y
    immutable: true,
    dev: true,
  });

  polka()
    .use(assets)
    .get('/data/commits', async (req, res) => {
      console.info(`Request: ${req.path}${req.search}`)
      const range = {
        start: req.query.from,
        end: req.query.to
      };
      if (range.start && range.end) {
        const data = await fetchCommits(range)
        send(res, 200, data);
      } else {
        console.warn("Argument to or from not defined:", range)
      }
    })
    .get('/data/projects', async (req, res) => {
      console.info(`Request projects: ${req.path}`)
      const data = await fetchProjectList()
      send(res, 200, data);
    })
    .listen(port, err => {
      if (err) throw err;
      console.log(`✨ Ready on localhost:${port}~ 🚀 !`);
    });
}