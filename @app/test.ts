import DashboardCardTest from './DashboardCard-test';
import SettingsTest from './Settings-test';
import CommitTest from './Commit-test';
import ProjectTest from './Project-test';
import HomeTest from './Home-test';
import NavbarTest from './Navbar-test';
import AppTest from './App-test';
import LoginTest from './Login-test';

console.debug('Initializing component tests')

const suites = {
  'DashboardCard': DashboardCardTest,
  'Settings': SettingsTest,
  'Commit': CommitTest,
  'Project': ProjectTest,
  'Home': HomeTest,
  'Navbar': NavbarTest,
  'App': AppTest,
  'Login': LoginTest,
}

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const canvas = document.getElementById('canvas')!

for (const suite in suites) {
  const tests = suites[suite as keyof typeof suites]
  console.debug(`Adding test suite for component ${suite} ${(tests as { name: string }[]).map(test => `'${test.name}'`)}`)
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
      console.debug(`Running '${test.name}' of ${suite} test suite`)
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
