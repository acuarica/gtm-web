import Settings from './Settings.svelte';

export default [{
  component: Settings,
  name: 'w/versions',
  props: {
    versions: {
      'Node': '1.2.3',
      'gtm': '4.5.6',
      'gtm-dash': '7.8.9',
    },
  },
}, {
  component: Settings,
  name: 'w/no versions',
  props: {
    versions: {
    },
  },
}]