
// import { Repository, Commit } from 'nodegit';

import { spawn } from 'child_process'


// import * as a from 'moment'
// const getMostRecentCommit = function (repo: Repository): Promise<Commit> {
//   return repo.getBranchCommit('develop');
// };

// const getCommitMessage = function (commit: Commit): string {
//   return commit.message();
// };

// Repository.open('.')
//   .then(getMostRecentCommit)
//   .then(getCommitMessage)
//   .then(function (message) {
//     console.log(message);
//   });

export async function fetchCommits(): Promise<string> {
  // const child = spawn('/usr/local/bin/gtm', ['report']);
  const child = spawn('ls');

  process.stdin.pipe(child.stdin)

  let buf = ""
  for await (const data of child.stdout) {
    console.log(`stdout from the child: ${data}`);
    buf += data
  };

  return `hello from gtm-git${buf}`

}