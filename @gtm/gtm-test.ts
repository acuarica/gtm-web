import { GitService, parseFileEntry } from '@gtm/gtm'
import { spawn } from 'child_process'
import assert from 'assert'
import { FileNote } from '@gtm/notes'

describe('@gtm/gtm', () => {

  const service = new GitService(
    (args: string[]) => spawn('node', ['@git/test/gtm-mock.js', ...args])
  )

  describe('fetchCommits', () => {

    it.skip('throws with invalid range', async () => {
      const range = { start: '1asdf', end: '2asdf' }
      assert.throws(() => service.fetchCommits(range), 'rejects because of invalid date')
    })

    it.skip('fetches empty commits', async () => {
      const range = { start: '2010-01-01', end: '2010-12-31' }
      const commits = await service.fetchCommits(range)
      assert.equal(commits.length, 0)
    })

    it.skip('fetches commits', async () => {
      const range = { start: '2020-04-01', end: '2020-05-01' }
      const commits = await service.fetchCommits(range)
      assert(commits.length > 0, 'no commits')
    })

  })

  it('detects invalid file entries', () => {
    assert(!parseFileEntry(''), 'empty file entry');
    assert(!parseFileEntry('src/file.ts:2797'), 'no timeline nor status given');
    assert(!parseFileEntry('src/file.ts:2797,m'), 'no timeline given');
    assert(!parseFileEntry('src/file.ts2797,1585861200:354,m'), 'invalid file separator');
    assert(!parseFileEntry('src/file.ts:123abc,1585861200:354,m'), 'invalid total time spent');
    assert(!parseFileEntry('src/file.ts:123,1585861200:354,a'), 'invalid status given');
  })

  it('parses file entries', () => {
    assert.deepStrictEqual(
      parseFileEntry('src/file.ts:150,1585861200:60,1585875600:90,m'),
      new FileNote(
        'src/file.ts',
        150,
        {
          '1585861200': 60,
          '1585875600': 90,
        },
        'm',
      ))
    assert.deepStrictEqual(
      parseFileEntry('comment/src/comment.ts:2797,1585861200:354,1585875600:50,1585879200:240,1585908000:444,1585918800:1629,1585929600:80,m'),
      new FileNote(
        'comment/src/comment.ts',
        2797,
        {
          '1585861200': 354,
          '1585875600': 50,
          '1585879200': 240,
          '1585908000': 444,
          '1585918800': 1629,
          '1585929600': 80,
        },
        'm',
      ), 'parse entry')

  })

  //   fn test_commit_note_invalid() {
  //     assert!(parse_commit_note(""));
  //     assert!(parse_commit_note("[]"));
  //     assert!(parse_commit_note("[ver:1total:213]"));
  //     assert!(parse_commit_note("[ver:a,total:213]"));
  //     assert!(parse_commit_note("[ver:1,total:a]"));
  //   }

  //   fn test_commit_note() {
  //     assert_eq!(
  //       parse_commit_note("[ver:2,total:213]").unwrap(),
  //       CommitNote {
  //       version: 2,
  //       total: 213,
  //       files: Vec:: new(),
  //     }
  //     );

  //     assert_eq!(
  //       parse_commit_note(
  //         "[ver:2,total:213]
  // closebrackets / src / closebrackets.ts: 950, 1585918800: 510, 1585922400: 400, 1585929600: 40, r
  // text / src / char.ts: 90, 1585918800: 90, r"
  //       )
  //         .unwrap(),
  //       CommitNote {
  //       version: 2,
  //       total: 213,
  //       files: vec![
  //         FileNote {
  //       source_file: "closebrackets/src/closebrackets.ts",
  //       time_spent: 950,
  //       timeline: [
  //         ("1585918800", 510),
  //         ("1585922400", 400),
  //         ("1585929600", 40),
  //       ]
  //         .iter()
  //         .cloned()
  //         .collect(),
  //       status: "r",
  //     },
  //       FileNote {
  //       source_file: "text/src/char.ts",
  //       time_spent: 90,
  //       timeline: [("1585918800", 90),].iter().cloned().collect(),
  //       status: "r",
  //     }
  //                 ],
  //             }
  //         );

  // let note = parse_commit_note("[ver:1,total:4037]
  // comment / src / comment.ts: 2797, 1585861200: 354, 1585875600: 50, 1585879200: 240, 1585908000: 444, 1585918800: 1629, 1585929600: 80, m
  // closebrackets / src / closebrackets.ts: 950, 1585918800: 510, 1585922400: 400, 1585929600: 40, r
  // text / src / char.ts: 90, 1585918800: 90, r
  // demo / demo.ts: 60, 1585918800: 60, r
  // state / src / selection.ts: 40, 1585918800: 40, r
  // highlight / src / highlight.ts: 30, 1585918800: 30, r
  // lang - javascript / src / javascript.ts: 30, 1585918800: 30, r
  // node_modules / w3c - keyname / index.d.ts: 20, 1585922400: 20, r
  // CHANGELOG.md: 20, 1585918800: 20, r").unwrap();

  //         assert_eq!(note.version, 1);
  // assert_eq!(note.total, 4037);
  // assert_eq!(note.files.len(), 9);
  // assert_eq!(
  //   note.files[3],
  //   FileNote {
  //   source_file: "demo/demo.ts",
  //   time_spent: 60,
  //   timeline: [("1585918800", 60)].iter().cloned().collect(),
  //   status: "r",
  // }
  // );
  //     }

  // it('git', async () => {
  //   const h = await gethistory()
  //   console.log(h)
  // })


})