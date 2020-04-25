import { colorSchemeSelect } from '../src/components'

colorSchemeSelect('color-scheme-picker').whenChange(select => {
  console.info("Color scheme selected:", select.value)
})