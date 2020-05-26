import Navbar from './Navbar.svelte';
import Settings from './Settings.svelte';

export default [{
  component: Navbar,
  name: 'w/title',
  props: {
    title: 'Title here',
    settingsView: Settings,
    settingsViewProps: { versions: { 'gtm': 'ver1', 'node': 'ver2', 'svelte': 'ver3' } },
  },
}, {
  component: Navbar,
  name: 'w/no title',
  props: {
    settingsView: Settings,
    settingsViewProps: { versions: { 'gtm': 'ver1', 'node': 'ver2', 'svelte': 'ver3' } },
  },
}]