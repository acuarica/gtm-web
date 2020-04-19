import { hhmm } from "./format";
import { Commit, ProjectMap, getProjectMap, DailyHours, getDaily } from "./gtm";
import { DropdownSelect, UI } from "./components";
import { Chart } from "chart.js"
import ChartDataLabels, { Context } from 'chartjs-plugin-datalabels';
import moment from 'moment';
import 'chartjs-plugin-colorschemes';

const ui = new UI()

function fetchjson(url: string, f: (response: any) => any) {
  fetch(`${url}${window.location.search}`)
    .then(data => data.json())
    .then(f)
}

const colorSelector = new DropdownSelect('color-scheme-select', [
  "tableau.Tableau10",
  "office.Excel16",
  "tableau.Tableau20",
  "tableau.Classic10",
  "tableau.ColorBlind10"])

fetchjson('/data/commits', (res: Commit[]) => {
  const projects: ProjectMap = getProjectMap(res)
  const daily: DailyHours = getDaily(projects)

  const labels: string[] = []
  const data: number[] = []
  const commitcounts: number[] = []
  for (const pname in projects) {
    const p = projects[pname]
    labels.push(pname)
    data.push(p.total)
    commitcounts.push(p.commitcount)
  }

  Chart.defaults.global.plugins!.colorschemes.scheme = colorSelector.value

  ui.newChart('projectTotalsChart', {
    type: 'doughnut',
    plugins: [ChartDataLabels],
    data: {
      datasets: [{
        data: data,
      }],
      labels: labels,
    },
    options: {
      maintainAspectRatio: false,
      title: { display: true, text: 'Reported time by Project' },
      legend: { position: 'left', },
      plugins: {
        datalabels: {
          formatter: (value: number, _context: Context) => hhmm(value),
        },
      },
      tooltips: {
        callbacks: {
          label: (tooltipItem, data) => {
            const i = tooltipItem.index!;
            const ds = data.datasets![0];
            const commitcount = commitcounts[i];
            const committext = commitcount == 1 ? 'commit' : 'commits';
            return `${data.labels![i]}: ${hhmm(ds.data![i] as number)} (${commitcount} ${committext})`;
          },
        }
      },
    }
  });

  const max = moment();
  const min = max.clone().subtract(7, 'day');

  ui.newChart('activityChart', {
    type: 'matrix',
    data: {
      datasets: Object.keys(projects).map(p => {
        return {
          label: projects[p].name,
          data: projects[p].timelineMatrix,
          borderWidth: 1,
          width: function (ctx: Context) {
            const value = (<{ v: number }>ctx.dataset.data![ctx.dataIndex]!).v;
            const levels = 10;
            const alpha = Math.floor(value * levels / 3600) / levels + (1 / levels);
            var a = ctx.chart.chartArea;
            return (a.right - a.left) / 25;
          },
          height: function (ctx: Context) {
            const value = (ctx.dataset.data![ctx.dataIndex]! as { v: number }).v;
            const levels = 4;
            const alpha = Math.floor(value * levels / 3600) / levels + (1 / levels);
            var a = ctx.chart.chartArea;
            return alpha * (a.bottom - a.top) / 10;
          },
        }
      }),
    },
    options: {
      maintainAspectRatio: false,
      scales: {
        xAxes: [{
          type: 'time',
          offset: true,
          time: { unit: 'hour', parser: 'HH:mm' },
          gridLines: { drawOnChartArea: false, },
        }],
        yAxes: [{
          type: 'time',
          offset: true,
          time: { unit: 'day', parser: 'YYYY-MM-DD' },
          ticks: {
            // reverse: true,
            // min: min.format('X'),
            // max: max.format('X'),
          },
          gridLines: { drawOnChartArea: false, },
        }, {
          type: 'time',
          offset: true,
          position: 'right',
          time: { unit: 'day', parser: 'YYYY-MM-DD' },
          ticks: {
            // reverse: true,
            callback: function (value, index, values: any) {
              const d = moment((values as { value: any }[])[index].value).format('YYYY-MM-DD');
              const date = daily[d];
              return date === undefined ? value : hhmm(date.total);
            }
          },
          gridLines: { drawOnChartArea: false, },
        }],
      },
      plugins: {
        datalabels: {
          display: false,
        },
        zoom: {
          pan: {
            enabled: true,
            mode: "y",
            speed: 100,
            threshold: 10,
          },
          zoom: {
            enabled: true,
            mode: 'y',
            speed: 0.1,
            sensitivity: 3,
          }
        },
      }
    }
  });

  colorSelector.whenChange((select: HTMLSelectElement) => {
    ui.charts.forEach(chart => {
      chart.options.plugins!.colorschemes.scheme = select.value;
      chart.update();
    })
  })

});
