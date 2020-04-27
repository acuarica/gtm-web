import { colorSchemeSelect } from '../src/components2'

import ChartPalettePicker from '../src/components/ChartPalettePicker.svelte'
new ChartPalettePicker({
  target: document.getElementById('svelte-placeholder2'),
  props: {
    // options: [{value:'asdf',data:{asdf:123}},{value:'asdasdff'}],
  },
});

colorSchemeSelect('color-scheme-picker').whenChange(select => {
  console.info("Color scheme selected:", select.value)
})
