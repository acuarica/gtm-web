import { pad0, hhmm } from "./format";
import * as Chart from "chart.js"
import * as DataLabels  from 'chartjs-plugin-datalabels';
import moment from 'moment';


// let user = "Jane User";
// export * from "./gtmweb";
// document.body.textContent = "asdf";
// document.body.textContent += new Unicorn().sayHelloTo("asdf");
// document.body.textContent += "asdfasdf";

// const colorSchemes = Chart.colorschemes;
// console.log(colorSchemes);
// Chart.plugins.unregister(DataLabels.ChartDataLabels);


const colors = {
  blue: 'rgb(54, 162, 235)',
};


function fetchjson(url: string, f: (response: any) => any) {
  fetch(`${url}${window.location.search}`)
    .then(data => data.json())
    .then(f)
}

function newchart(chartid: string, config: Chart.ChartConfiguration) {
  const canvas = <HTMLCanvasElement>document.getElementById(chartid)
  const ctx = canvas.getContext('2d')
  new Chart.Chart(ctx!, config)
}

type Project = {
  name: string,
  total: number,
  commitcount: number,
  timeline: {
    [id: string]: {
      [hour: number]: {
        total: number
      }
    }
  },
  timelineMatrix: { x: string, y: string, v: number }[],
}

type Commit = {
  Project: string,
  Note: {
    Files: {
      Timeline: { [id: number]: number },
      TimeSpent: number,
    }[]
  }
  When: string,
}

fetchjson('/data/commits', (res: Commit[]) => {
  const projects: { [id: string]: Project } = {};
  for (const commit of res) {
    let project = projects[commit.Project];
    if (project === undefined) {
      project = { name: commit.Project, total: 0, commitcount: 0, timeline: {}, timelineMatrix: [] };
      projects[commit.Project] = project;
    }
    project.commitcount++;
    if (commit.Note.Files === null) {
      console.warn("gtm check: Commit note files not available:", commit);
      continue;
    }
    for (const file of commit.Note.Files) {
      let filesecs = 0;
      for (let timestamp2 in file.Timeline) {
        const timestamp = Number(timestamp2)
        const secs = file.Timeline[timestamp];
        if (secs > 3600) console.warn("gtm check: Duration (in seconds) should be less than 3600:", secs);
        if (timestamp % 3600 !== 0) console.warn("gtm check: Timestamp (unix time) should be by the hour:", timestamp);
        project.total += secs;
        const date = moment.unix(timestamp).startOf('day').format('YYYY-MM-DD');
        const hour = moment.unix(timestamp).hour();
        let dateline = project.timeline[date];
        if (dateline === undefined) {
          dateline = {};
          project.timeline[date] = dateline;
        }
        let hourline = dateline[hour];
        if (hourline === undefined) {
          hourline = { total: 0 };
          dateline[hour] = hourline;
        }
        hourline.total += secs;
        filesecs += secs;
      }
      if (filesecs !== file.TimeSpent) console.warn("gtm check: Timeline seconds does not add up to duration in file.");
    }
  }
  const daily: { [date: string]: { total: number } } = {};
  for (const pkey in projects) {
    const p = projects[pkey]
    const data = [];
    for (const date in p.timeline) {
      for (const h in p.timeline[date]) {
        const hour = Number(h)
        const secs = p.timeline[date][hour];
        data.push({
          x: `${pad0(hour)}:00`, y: date, v: secs.total,
        });
        let day = daily[date];
        if (day === undefined) {
          day = { total: 0 };
          daily[date] = day;
        }
        day.total += secs.total;
      }
    }
    p.timelineMatrix = data;
  }

  newchart('projectTotalsChart', {
    type: 'doughnut',
    // plugins: [DataLabels],
    data: {
      datasets: [{
        data: Object.keys(projects).map(x => projects[x].total),
        // commitcounts: Object.keys(projects).map(x => projects[x].commitcount),
      }],
      labels: Object.keys(projects),
    },
    options: {
      maintainAspectRatio: false,
      title: { display: true, text: 'Reported time by Project' },
      legend: { position: 'left', },
      plugins: {
        // colorschemes: { scheme: 'office.BlackTie6' },
        datalabels: {
          display: 'auto',
          formatter: (value, context) => hhmm(value),
        },
      },
      tooltips: {
        callbacks: {
          label: (tooltipItem, data) => {
            const i = tooltipItem.index;
            const ds = data.datasets![0];
            // const commitcount = ds.commitcounts[i];
            // const committext = commitcount == 1 ? 'commit' : 'commits';
            // return `${data.labels[i]}: ${hhmm(ds.data[i])} (${commitcount} ${committext})`;
            return "";
          },
        }
      },
    }
  });

  const max = moment();
  const min = max.clone().subtract(7, 'day');
  // console.log(min, max);
  // console.log(min.format('X'), max.format('X'));

  newchart('activityChart', {
    type: 'matrix',
    data: {
      datasets: Object.keys(projects).map(p => {
        return {
          label: projects[p].name,
          data: projects[p].timelineMatrix,
          borderWidth: 1,
          width: function (ctx: DataLabels.Context) {
            const value = (<{ v: number }>ctx.dataset.data![ctx.dataIndex]!).v;
            const levels = 10;
            const alpha = Math.floor(value * levels / 3600) / levels + (1 / levels);
            var a = ctx.chart.chartArea;
            return (a.right - a.left) / 25;
          },
          height: function (ctx: DataLabels.Context) {
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
            callback: function (value, index, values) {
              // const d = moment(values[index].value).format('YYYY-MM-DD');
              // const d = moment(values[index]).format('YYYY-MM-DD');
              // const date = daily[d];
              // return date === undefined ? value : hhmm(date.total);
              return "";
            }
          },
          gridLines: { drawOnChartArea: false, },
        }],
      },
      plugins: {
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
