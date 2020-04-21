import { Chart } from "chart.js"
import { Commit, timeSpent } from "./gtm"
import { hhmm } from "./format"

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
  const id = `collapse-${commit.Hash}`
  return `<a class="list-group-item list-group-item-action btn btn-primary" href="#${id}" data-toggle="collapse" role="button" aria-expanded="true" aria-controls="${id}">
            <div class="d-flex w-100 justify-content-between">
              <div class="mb-2">
                <span class="badge badge-pill badge-primary">${commit.Project}</span>
              </div>
              <div class="mb-2">
                <span class="badge badge-pill badge-light text-muted">${commit.Author}</span>
              </div>
              <small class="mb-2 text-muted">
                <i class="fas fa-clock"></i>
                ${hhmm(timeSpent(commit))}
              </small>
              <small class="text-muted">
                <i class="fa fa-calendar"></i>&nbsp;
                ${commit.When}
              </small>
            </div>
            <h6 class="mb-1">${commit.Subject}</h6>
            <small class="mb-1">${commit.Message.replace('\n', '<br>')}</small>
            <div class="collapse" id="${id}">
              <ul>${commit.Note.Files.map(file => 
                `<li class="small">${file.SourceFile} &nbsp; 
                  <i class="fas fa-clock"></i> ${hhmm(file.TimeSpent)}
                </li>`)
                .join('')}</ul>
            </div>
          </a>`
}