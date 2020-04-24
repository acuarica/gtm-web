import { colorSchemeSelect } from '../src/color-scheme-select'

colorSchemeSelect('color-scheme-picker').whenChange(select => {
  console.info("Color scheme selected:", select.value)
  const input = document.getElementById('color-scheme-output') as HTMLInputElement
  input.value = select.value
})