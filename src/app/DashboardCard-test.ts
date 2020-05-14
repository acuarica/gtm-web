import DashboardCard from './DashboardCard.svelte';

// eslint-disable-next-line @typescript-eslint/explicit-function-return-type
export default [{
  component: DashboardCard,
  name: 'Full card',
  props: {
    title: 'Title here',
    body: 'Here comes the body',
    footer: 'Last but not least the footer'
  },
}]