import Commit from '../app/Commit.svelte';

export default [{
  component: Commit,
  name: 'showing full commit',
  props: {
    commit: {
      Author: 'me',
      Project: 'gtm web',
      Subject: 'Short message',
      Message: 'Long message',
      When: 'today',
      timeSpent: 2 * 60,
      Note: {
        Files: [
          { SourceFile: 'src/test.rs', TimeSpent: 3 * 60 * 60 }
        ]
      }
    },
  },
}, {
  component: Commit,
  name: 'w/no commit message',
  props: {
    commit: {
      Author: 'you',
      Project: 'gtm web',
      Subject: 'Short message with no long message',
      When: 'tomorrow',
      timeSpent: 120 * 60 * 60 + 34 * 60,
    },
  },
}]