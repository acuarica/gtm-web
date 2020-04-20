import { Chart } from "chart.js"
import { Commit } from "./gtm"

export class UI {

  readonly charts: Chart[] = []

  newChart(chartid: string, config: Chart.ChartConfiguration) {
    const canvas = <HTMLCanvasElement>document.getElementById(chartid)
    const ctx = canvas.getContext('2d')
    const chart = new Chart(ctx!, config)
    this.charts.push(chart)
  }

}

export class DropdownSelect {

  private readonly select: HTMLSelectElement

  constructor(selectId: string, options: string[]) {
    this.select = <HTMLSelectElement>document.getElementById(selectId)
    for (const text of options) {
      const option = document.createElement('option')
      option.text = text
      this.select.options.add(option)
    }
  }

  get value(): string {
    return this.select.value
  }

  whenChange(listener: (select: HTMLSelectElement) => any) {
    this.select.addEventListener('change', function (this, _event) {
      listener(this)
    })
  }

}

export function getCommitElement(commit: Commit): string {
  const A = commit.Author[0] ?? ""
  const contact = `<span class="mdl-chip mdl-chip--contact">
    <span class="mdl-chip__contact mdl-color--teal mdl-color-text--white">${A}</span>
    <span class="mdl-chip__text">${commit.Author}</span>
    </span>`
  const date = `<i class="material-icons">event</i> ${commit.When}`

  const file = (f: string) => `<li class="mdl-list__item">
    <span class="mdl-list__item-primary-content">
      ${f}adsfa
    </span>
  </li>`

  const x = commit.Note.Files.map(f => file(f.SourceFile)).join()
  console.log(x)
  const files = `<ul class="demo-list-item mdl-list">
<li class="mdl-list__item">
    <span class="mdl-list__item-primary-content">
    item 1
    </span>
  </li>
  ${x}
</ul>`

  return `<div class="demo-card-wide mdl-shadow--2dp">
            <div class="mdl-card__title">
              ${contact}
              <div class="mdl-layout-spacer"></div>
              ${date}
            </div>
            <div class="mdl-card__title">${commit.Subject}</div>
            <div class="mdl-card__supporting-text">${commit.Message}</div>
            asdf
            ${files}
            asdf
          </div>`
}