import { colorSchemeSelect } from '../src/components'

colorSchemeSelect('color-scheme-picker').whenChange(select => {
  console.info("Color scheme selected:", select.value)
})


customElements.define('chart-color-picker', class extends HTMLElement {
  constructor() {
    super()

    const color = this.getAttribute('color')
    // const shadowDOM = this.attachShadow({ mode: 'open' })

    const select = document.createElement('select')
    select.id = 'hola'
    select.className = 'selectpicker'
    // text.innerHTML = this.innerHTML
    // this.innerHTML = ''

    // const style = document.createElement('style')
    // style.innerHTML = `p { color: ${color} }`

    // shadowDOM.appendChild(style)
    // shadowDOM.appendChild(select)
    this.appendChild(select)


    colorSchemeSelect('hola')
  }

})