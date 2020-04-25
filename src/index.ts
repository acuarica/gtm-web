import { Commit, getProjectMap, DailyHours, getDaily, FileStatus } from "./gtm";
import { UI, getCommitElement, colorSchemeSelect } from "./components";
import { Chart } from "chart.js"
import moment from 'moment';
import $ from 'jquery';
import 'daterangepicker'
import { projectTotalsChartConfig, activityChartConfig, timeByFileStatusChartConfig } from "./charts";
import { hhmm } from "./format";
import 'bootstrap'
import 'bootstrap-select'

let commitsDataUrl: string
if (process.env.NODE_ENV === 'development') {
  commitsDataUrl = '/data/commits'
} else {
  commitsDataUrl = '/gtm-web/data-commits.json'
}

const ui = new UI()

function fetchjson(url: string, handler: (response: Commit[]) => void): void {
  $('#progress').show()
  fetch(url)
    .then(data => data.json())
    .then(response => {
      $('#progress').hide()
      handler(response)
    })
}

const colorSelector = colorSchemeSelect('color-scheme-picker')

Chart.defaults.global.plugins!.colorschemes.scheme = colorSelector.value

let totalTimeChart: Chart | null = null
let pchart: Chart | null = null
// let _achart: Chart | null = null

const StatusIndicator: FileStatus<string> = {'m': 'Modify', 'r': 'Read', 'd': 'Delete'}

function fetchCommits(from: string, to: string): void {
  const nav = window.location.search.length == 0 ? "?" : window.location.search
  console.log(window.location.search)
  const url = `${commitsDataUrl}${nav}&from=${from}&to=${to}`

  fetchjson(url, (res: Commit[]) => {
    const { projects, totalSecs, status } = getProjectMap(res)
    const daily: DailyHours = getDaily(projects)

    document.getElementById('totalSecs')!.innerText = hhmm(totalSecs)
    document.getElementById('totalNoCommits')!.innerText = `${res.length}`

    const e = document.getElementById('commitsPlaceholder')
    for (const c of res.sort((c, d) => c.When >= d.When ? 1 : -1)) {
      if (c.Note.Files.length == 0) continue
      e!.insertAdjacentHTML('afterend', getCommitElement(c))
    }
    $('.collapse').collapse('hide')

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
      totalTimeChart = ui.newChart('totalTimeChart', timeByFileStatusChartConfig());
      pchart = ui.newChart('projectTotalsChart', projectTotalsChartConfig());
      // _achart = 
      ui.newChart('activityChart', activityChartConfig(projects, daily));
    }
    // totalTimeChart!.data.datasets = [{ data: [20], label: "123"}]
    totalTimeChart!.data.datasets = Object.keys(status).map(s => {
      return {
        data: [status[s]],
        label: `${StatusIndicator[s]}: ${hhmm(status[s])}`
      }
    })
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
