import { colorSchemeSelect } from './src/color-scheme-select'

colorSchemeSelect('color-scheme-picker').whenChange(select => {
  console.info("Color scheme selected:", select.value)
  const input = <HTMLInputElement>document.getElementById('color-scheme-output')
  input.value = select.value
})