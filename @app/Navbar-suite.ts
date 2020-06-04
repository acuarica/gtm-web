import Navbar from './Navbar.svelte';
import Settings from './Settings.svelte';

export default [{
  name: 'w/title',
  props: {
    title: 'Title here',
  },
}, {
  name: 'w/no title',
  props: {
  },
}].map(e => {
  return {
    component: Navbar, name: e.name, props: {
      settingsView: class extends Settings {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        constructor(opts: { target: Element; props?: Record<string, any> | undefined }) {
          super({
            ...opts, props: { versions: { 'gtm': 'ver1', 'node': 'ver2', 'svelte': 'ver3' } }
          })
        }
      },
      ...e.props,
    }
  }
})