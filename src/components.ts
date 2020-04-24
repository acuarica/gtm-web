import { Chart } from "chart.js"
import { Commit, timeSpent } from "./gtm"
import { hhmm } from "./format"

export class UI {

  readonly charts: Chart[] = []

  newChart(chartid: string, config: Chart.ChartConfiguration): Chart {
    const canvas = document.getElementById(chartid) as HTMLCanvasElement
    console.assert(canvas, `Chart canvas element '${chartid}' not found`)
    const ctx = canvas.getContext('2d')
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const chart = new Chart(ctx!, config)
    this.charts.push(chart)
    return chart
  }

}

export class DropdownSelect {

  private readonly select: HTMLSelectElement

  constructor(selectId: string, options: { value: string; data: { [key: string]: string } }[]) {
    this.select = document.getElementById(selectId) as HTMLSelectElement
    console.assert(this.select, `Element '${selectId}' must be of type 'HTMLSelectElement', but got`, this.select)
    for (const { value, data } of options) {
      const option = document.createElement('option')
      option.text = value
      for (const key in data) {
        const value = data[key];
        option.setAttribute(`data-${key}`, value)
      }
      this.select.options.add(option)
    }
  }

  get value(): string {
    return this.select.value
  }

  whenChange(listener: (select: HTMLSelectElement) => void): void {
    this.select.addEventListener('change', function (this, ) {
      listener(this)
    })
  }

}

export function colorSchemeSelect(selectId: string): DropdownSelect {
  return new DropdownSelect(selectId, [
    "tableau.Tableau10",
    "office.Excel16",
    "tableau.Tableau20",
    "tableau.Classic10",
    "tableau.ColorBlind10"].map(e => {
      const [group, pallete] = e.split('.')
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const colorSchemes = (Chart as any).colorschemes as {
        [group: string]: { [pallete: string]: string[] };
      }
      return {
        value: e,
        data: {
          width: "400px",
          content: `<div><div style="width: 80px; display: inline-block"><small class="text-muted">${pallete}</small></div>
          ${colorSchemes[group][pallete].map(color =>
            `<div style="background-color: ${color}; width: 12px; display: inline-block">&nbsp;</div>`).join('')}
          </div>`,
        }
      }
    })
  )
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

import 'chartjs-plugin-colorschemes';
import 'bootstrap'
import 'bootstrap-select'