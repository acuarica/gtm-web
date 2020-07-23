import moment from 'moment';
import ChartDataLabels, { Context } from 'chartjs-plugin-datalabels';
import { getDaily, FileStatus, ProjectList } from '@gtm/notes';
import { hhmm } from '@gtm/notes';
import { ChartConfiguration } from 'chart.js'

export function timeByFileStatusChartConfig(status: FileStatus<number>): ChartConfiguration {
  const StatusIndicator: FileStatus<string> = {
    m: 'Modifying',
    r: 'Reading',
    d: 'Deleting',
  };

  return {
    type: 'horizontalBar',
    data: {
      datasets: Object.keys(status).map(s => {
        return {
          data: [status[s]],
          label: `${StatusIndicator[s]}: ${hhmm(status[s])}`,
        };
      }),
    },
    options: {
      maintainAspectRatio: false,
      title: {
        display: true,
        text: 'Time by Activity',
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
        }],
      },
      plugins: {
        datalabels: {
          display: false,
        },
      },
      tooltips: {
        enabled: false,
      },
    },
  }
}

///
export function projectTotalsChartConfig(projects: ProjectList): ChartConfiguration {
  console.assert(typeof projects === 'object', 'Invalid projects:', projects)
  const datasets: { data: number[]; commitcount: number; label: string }[] = [];
  for (const pname in projects) {
    const p = projects[pname];
    datasets.push({
      data: [p.total],
      commitcount: p.commits.length,
      label: pname,
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
        text: 'Time by Project',
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
        }],
      },
      plugins: {
        datalabels: {
          formatter: (value): string => `\n${hhmm(value)}`,
        },
      },
      tooltips: {
        enabled: false,
      },
    },
  }
}

///
export function activityChartConfig(projects: ProjectList, status = false): ChartConfiguration {
  const title = status
    ? 'Uncommited timeline by Project (status)'
    : 'Committed timeline by Project (report)'
  const daily = getDaily(projects)
  const dailyKeys = Object.keys(daily)
  return {
    type: 'matrix',
    data: {
      datasets: Object.keys(projects).map(p => {
        return {
          label: projects[p].name,
          data: projects[p].timelineMatrix,
          borderWidth: 1,
          width: function (ctx: Context): number {
            const a = ctx.chart.chartArea;
            return (a.right - a.left) / 25;
          },
          height: function (ctx: any): number {
            const value = (ctx.dataset.data[ctx.dataIndex]).v;
            const levels = 4;
            const alpha = Math.floor(value * levels / 3600) / levels + (1 / levels);
            const a = ctx.chart.chartArea;
            return alpha * (a.bottom - a.top) / dailyKeys.length;
          },
        }
      }),
    },
    options: {
      maintainAspectRatio: false,
      title: { display: true, text: title },
      scales: {
        xAxes: [{
          type: 'time',
          offset: true,
          time: {
            unit: 'hour', parser: 'HH:mm',
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
          },
          gridLines: { drawOnChartArea: false },
        }, {
          type: 'time',
          offset: true,
          position: 'right',
          time: {
            unit: 'day', parser: 'YYYY-MM-DD',
          },
          ticks: {
            callback: function (_value, index, values): string {
              const d = moment((values[index] as any).value).format('YYYY-MM-DD');
              const date = daily[d];
              return date === undefined ? '' : hhmm(date.total);
            },
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
      },
    },
  }
}

import 'chartjs-chart-matrix';