import { spawn } from 'child_process'

export async function fetchCommits(range: { start: string; end: string }): Promise<any> {
  // const child = spawn('/usr/local/bin/gtm', ['report']);
  const gtmexec = '/Users/luigi/work/#forks/gtm/bin/gtm'
  const args = [
    'export',
    '-data=commits',
    `-from-date=${range.start}`,
    `-to-date=${range.end}`]
  const child = spawn(gtmexec, args);

  child.on('exit', code => {
    console.log(`Exit code is: ${code}`);
  });

  let buf = ""
  for await (const data of child.stdout) {
    buf += data
  };

  return JSON.parse(buf)

}