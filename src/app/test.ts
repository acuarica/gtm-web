import DashboardCardTest from './DashboardCard-test';
import CommitTest from './Commit-test';
import ProjectTest from './Project-test';
import HomeTest from './Home-test';
import AppTest from './App-test';

console.info('Initializing component tests')

const suites = {
  'DashboardCard': DashboardCardTest,
  'Commit': CommitTest,
  'Project': ProjectTest,
  'Home': HomeTest,
  'App': AppTest,
}

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const canvas = document.getElementById('canvas')!

for (const suite in suites) {
  const tests = suites[suite as keyof typeof suites]
  console.info(`Adding test suite for component ${suite} with test cases ${(tests as { name: string }[]).map(test => `'${test.name}'`)}`)
  const suiteDiv = document.createElement('div')
  suiteDiv.textContent = suite
  suiteDiv.classList.add('text-gray-500', 'mt-1')
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  document.getElementById('menu')!.appendChild(suiteDiv)

  for (const test of tests) {
    const item = document.createElement('button')
    item.textContent = test.name
    item.classList.add('block', 'ml-1', 'text-sm')
    item.addEventListener('click', async () => {
      console.info(`Running '${test.name}' of ${suite} test suite`)
      canvas.innerHTML = ''
      new test.component({
        target: canvas,
        props: test.props,
      })
    })

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    document.getElementById('menu')!.appendChild(item)
  }
}
