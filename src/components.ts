import { Chart } from "chart.js"

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