import { pad0, hhmm } from "./format";
import { Commit, ProjectMap, getProjectMap, DailyHours, getDaily } from "./gtm";
// import * as Chart from "chart.js"
import { Chart } from "chart.js"
// import * as DataLabels  from 'chartjs-plugin-datalabels';
import { Context } from 'chartjs-plugin-datalabels';
import moment from 'moment';

// const colorSchemes = Chart.colorschemes;
// console.log(colorSchemes);

function fetchjson(url: string, f: (response: any) => any) {
  fetch(`${url}${window.location.search}`)
    .then(data => data.json())
    .then(f)
}

function newchart(chartid: string, config: Chart.ChartConfiguration) {
  const canvas = <HTMLCanvasElement>document.getElementById(chartid)
  const ctx = canvas.getContext('2d')
  new Chart(ctx!, config)
}

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
  newchart('projectTotalsChart', {
    type: 'doughnut',
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
        colorschemes: { scheme: 'office.BlackTie6' },
        datalabels: {
          display: 'auto',
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

  newchart('activityChart', {
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
        colorschemes: { scheme: 'office.BlackTie6' },
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
});
