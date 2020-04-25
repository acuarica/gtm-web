import moment from 'moment';
import { ChartConfiguration } from "chart.js";
import ChartDataLabels, { Context } from 'chartjs-plugin-datalabels';
import { hhmm } from "./format";
import { ProjectMap, DailyHours } from "./gtm";

export function totalTimeChartConfig(): ChartConfiguration {
  return {
    type: 'horizontalBar',
    // plugins: [ChartDataLabels],
    data: {
      datasets: [{
        // data: [],
        label: "asdf",
        backgroundColor: '#17a2b8', //!important
      }],
      // labels: ['sddsd']
    },
    options: {
      maintainAspectRatio: false,
      title: {
        display: true,
        text: 'Time by Activity'
      },
      legend: {
        position: 'bottom',
      },
      scales: {
        xAxes: [{
          display: false,
          stacked: true,
        }],
        yAxes: [{
          stacked: true,
          gridLines: {
            display: false,
          },
        }]
      },
    },
  }
}

///
export function projectTotalsChartConfig(): ChartConfiguration {
  return {
    // type: 'horizontalBar',
    type: 'pie',
    plugins: [ChartDataLabels],
    data: {
      // datasets: datasets,
      labels: ['Total by\nProject'.split('\n')],
    },
    options: {
      maintainAspectRatio: false,
      title: {
        display: true,
        text: 'Reported time by Project'
      },
      legend: {
        position: 'top',
        labels: {
          generateLabels: function (chart) {
            return [{
              text: "asd"
            }]

          }
        },
      },
      scales: {
        xAxes: [{
          display: false,
          stacked: true,
        }],
        yAxes: [{
          stacked: true,
          gridLines: {
            display: false,
          },
        }]
      },
      plugins: {
        datalabels: {
          formatter: (value: number, _context: Context) => hhmm(value),
        },
      },
      tooltips: {
        callbacks: {
          label: (_tooltipItem, _data) => {
            return ''
            // const i = tooltipItem.index!;
            // const ds = data.datasets![0];
            // const commitcount = commitCounts[i];
            // const committext = commitcount == 1 ? 'commit' : 'commits';
            // return `${data.labels![i]}: ${hhmm(ds.data![i] as number)} (${commitcount} ${committext})`;
          },
        }
      },
    }
  }
}

///
export function activityChartConfig(projects: ProjectMap, daily: DailyHours): ChartConfiguration {
  return {
    type: 'matrix',
    data: {
      datasets: Object.keys(projects).map(p => {
        return {
          label: projects[p].name,
          data: projects[p].timelineMatrix,
          borderWidth: 1,
          width: function (ctx: Context) {
            // const value = (<{ v: number }>ctx.dataset.data![ctx.dataIndex]!).v;
            // const levels = 10;
            // const alpha = Math.floor(value * levels / 3600) / levels + (1 / levels);
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
      title: { display: true, text: 'Reported timeline by Project' },
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
            callback: function (_value, index, values: any) {
              const d = moment((values as { value: any }[])[index].value).format('YYYY-MM-DD');
              const date = daily[d];
              return date === undefined ? "" : hhmm(date.total);
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
  }
}

import 'chartjs-chart-matrix';
import 'chartjs-plugin-zoom';