import { Commit, ProjectMap, getProjectMap, DailyHours, getDaily } from "./gtm";
import { UI, getCommitElement, colorSchemeSelect } from "./components";
import { Chart } from "chart.js"
import moment from 'moment';
import $ from 'jquery';
// import 'chartjs-plugin-colorschemes';
import 'daterangepicker'
import { projectTotalsChartConfig, activityChartConfig, totalTimeChartConfig } from "./charts";

let commitsDataUrl: string
if (process.env.NODE_ENV === 'development') {
  commitsDataUrl = '/data/commits'
} else {
  commitsDataUrl = '/gtm-web/data-commits.json'
}

const ui = new UI()


function fetchjson(url: string, f: (response: any) => any) {
  fetch(url)
    .then(data => data.json())
    .then(f)
}


const colorSelector = colorSchemeSelect('color-scheme-picker')

Chart.defaults.global.plugins!.colorschemes.scheme = colorSelector.value

let totalTimeChart: Chart | null = null
let pchart: Chart | null = null
// let _achart: Chart | null = null


const pdiv = document.getElementById("progress") as HTMLDivElement

function fetchCommits(from: string, to: string) {
  const nav = window.location.search.length == 0 ? "?" : window.location.search
  console.log(window.location.search)
  const url = `${commitsDataUrl}${nav}&from=${from}&to=${to}`

  pdiv.hidden = false
  fetchjson(url, (res: Commit[]) => {
    const projects: ProjectMap = getProjectMap(res)
    const daily: DailyHours = getDaily(projects)

    const e = document.getElementById('commitsPlaceholder')
    for (const c of res.sort((c, d) => c.When >= d.When ? 1 : -1)) {
      if (c.Note.Files.length == 0) continue
      e!.insertAdjacentHTML('afterend', getCommitElement(c))
    }
    $('.collapse').collapse('hide')


    pdiv.hidden = true

    const commitCounts: number[] = []
    const datasets = []
    for (const pname in projects) {
      const p = projects[pname]
      commitCounts.push(p.commitcount)
      datasets.push({
        data: [p.total],
        label: pname,
      })
    }

    if (pchart == null) {
      totalTimeChart = ui.newChart('totalTimeChart', totalTimeChartConfig());
      pchart = ui.newChart('projectTotalsChart', projectTotalsChartConfig());
      // _achart = 
      ui.newChart('activityChart', activityChartConfig(projects, daily));
    }
    // totalTimeChart!.data.datasets = [{ data: [20], label: "123"}]
    totalTimeChart!.data.datasets![0].data = [12]// = [{ data: [20], label: "123"}]
    totalTimeChart!.update()

    pchart.data.datasets = datasets
    pchart.update()
    // ui.charts.forEach(chart => chart.update)

    colorSelector.whenChange(select => {
      ui.charts.forEach(chart => {
        chart.options.plugins!.colorschemes.scheme = select.value;
        chart.update();
      })
    })
  });
}


$(function () {

  var start = moment().subtract(29, 'days');
  var end = moment();

  function cb(start: any, end: any) {
    $('#reportrange span').html(start.format('MMMM D, YYYY') + ' - ' + end.format('MMMM D, YYYY'));
    console.log('asdf')
    fetchCommits(start.format('YYYY-MM-DD'), end.format('YYYY-MM-DD'));
  }

  $('#reportrange').daterangepicker({
    startDate: start,
    endDate: end,
    ranges: {
      'Today': [moment(), moment()],
      'Yesterday': [moment().subtract(1, 'days'), moment().subtract(1, 'days')],
      'Last 7 Days': [moment().subtract(6, 'days'), moment()],
      'Last 30 Days': [moment().subtract(29, 'days'), moment()],
      'This Month': [moment().startOf('month'), moment().endOf('month')],
      'Last Month': [moment().subtract(1, 'month').startOf('month'), moment().subtract(1, 'month').endOf('month')]
    }
  }, cb);

  cb(start, end);

});

import 'bootstrap'
import 'bootstrap-select'
