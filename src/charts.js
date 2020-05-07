import moment from 'moment';
import ChartDataLabels from 'chartjs-plugin-datalabels';
import { getDaily } from "@gtm/notes";
import { hhmm } from "@gtm/format";

export function timeByFileStatusChartConfig(status) {
  const StatusIndicator = { m: "Modifying", r: "Reading", d: "Deleting" };
  const datasets = Object.keys(status).map(s => {
    return {
      data: [status[s]],
      label: `${StatusIndicator[s]}: ${hhmm(status[s])}`
    };
  });

  return {
    type: 'horizontalBar',
    data: {
      datasets: datasets,
    },
    options: {
      maintainAspectRatio: true,
      title: {
        display: true,
        text: 'Time by Activity'
      },
      legend: {
        position: 'top',
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
          display: false,
        },
      },
      tooltips: {
        enabled: false,
      }
    },
  }
}

///
export function projectTotalsChartConfig(projects) {
  console.assert(typeof projects === 'object', `Invalid projects:`, projects)
  const datasets = [];
  for (const pname in projects) {
    const p = projects[pname];
    datasets.push({
      data: [p.total],
      commitcount: p.commits.length,
      label: pname
    });
  }

  return {
    type: 'bar',
    plugins: [ChartDataLabels],
    data: {
      datasets: datasets,
    },
    options: {
      maintainAspectRatio: false,
      title: {
        display: true,
        text: 'Time by Project'
      },
      legend: {
        position: 'top',
      },
      scales: {
        yAxes: [{
          display: false,
          gridLines: {
            display: false,
          },
        }]
      },
      plugins: {
        datalabels: {
          formatter: (value, _ctx) => `\n${hhmm(value)}`,
        },
      },
      tooltips: {
        enabled: false,
      },
    }
  }
}

///
export function activityChartConfig(projects) {
  const daily = getDaily(projects)
  const dailyKeys = Object.keys(daily)
  console.log(projects, daily, dailyKeys)
  return {
    type: 'matrix',
    data: {
      datasets: Object.keys(projects).map(p => {
        return {
          label: projects[p].name,
          data: projects[p].timelineMatrix,
          borderWidth: 1,
          width: function (ctx) {
            // const value = (<{ v: number }>ctx.dataset.data![ctx.dataIndex]!).v;
            // const levels = 10;
            // const alpha = Math.floor(value * levels / 3600) / levels + (1 / levels);
            var a = ctx.chart.chartArea;
            return (a.right - a.left) / 25;
          },
          height: function (ctx) {
            const value = (ctx.dataset.data[ctx.dataIndex]).v;
            const levels = 4;
            const alpha = Math.floor(value * levels / 3600) / levels + (1 / levels);
            var a = ctx.chart.chartArea;
            return alpha * (a.bottom - a.top) / dailyKeys.length;
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
          time: {
            unit: 'hour', parser: 'HH:mm'
          },
          gridLines: {
            drawOnChartArea: false,
          },
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
          time: {
            unit: 'day', parser: 'YYYY-MM-DD'
          },
          ticks: {
            callback: function (_value, index, values) {
              const d = moment(values[index].value).format('YYYY-MM-DD');
              const date = daily[d];
              return date === undefined ? "" : hhmm(date.total);
            }
          },
          gridLines: {
            drawOnChartArea: false,
          },
        }],
      },
      plugins: {
        datalabels: {
          display: false,
        },
      }
    }
  }
}

import 'chartjs-chart-matrix';