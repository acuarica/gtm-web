// import $ from 'jquery';
import Chart from 'chart.js';
import { DropdownSelect } from './components'
import 'chartjs-plugin-colorschemes';

export function colorSchemeSelect(selectId: string, className: string = "form-control"): DropdownSelect {
  return new DropdownSelect(selectId, `selectpicker ${className}`, [
    "tableau.Tableau10",
    "office.Excel16",
    "tableau.Tableau20",
    "tableau.Classic10",
    "tableau.ColorBlind10"].map(e => {
      const [group, pallete] = e.split('.')
      const colorSchemes = (Chart as any).colorschemes as {
        [group: string]: { [pallete: string]: string[] }
      }
      return {
        value: e,
        data: {
          content: `<div><div style="width: 125px; display: inline-block"><small class="text-muted">${pallete}</small></div>
          ${colorSchemes[group][pallete].map(color =>
            `<div style="background-color: ${color}; width: 12px; display: inline-block">&nbsp;</div>`).join('')}
          </div>`,
        }
      }
    })
  )
}

import 'bootstrap'
import 'bootstrap-select'
