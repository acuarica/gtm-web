import { Commit, ProjectMap, getProjectMap, DailyHours, getDaily } from "./gtm";
import { UI, getCommitElement } from "./components";
import { Chart } from "chart.js"
import moment from 'moment';
import $ from 'jquery';
// import 'chartjs-plugin-colorschemes';
import 'daterangepicker'
import { projectTotalsChartConfig, activityChartConfig } from "./charts";
import { colorSchemeSelect } from "./color-scheme-select";

let commitsDataUrl: string
if (process.env.NODE_ENV === 'development') {
  commitsDataUrl = '/data/commits'
} else {
  commitsDataUrl = '/gtm-web/data-commits.json'
}

const ui = new UI()

$(function () {

  var start = moment().subtract(29, 'days');
  var end = moment();

  function cb(start: any, end: any) {
    $('#reportrange span').html(start.format('MMMM D, YYYY') + ' - ' + end.format('MMMM D, YYYY'));
    console.log('asdf')
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


function fetchjson(url: string, f: (response: any) => any) {
  fetch(`${url}${window.location.search}`)
    .then(data => data.json())
    .then(f)
}

const colorSelector = colorSchemeSelect('color-scheme-picker')

fetchjson(commitsDataUrl, (res: Commit[]) => {
  const projects: ProjectMap = getProjectMap(res)
  const daily: DailyHours = getDaily(projects)

  const e = document.getElementById('commitsPlaceholder')
  for (const c of res.sort((c, d) => c.When >= d.When ? 1 : -1)) {
    if (c.Note.Files.length == 0) continue
    e!.insertAdjacentHTML('afterend', getCommitElement(c))
  }
  $('.collapse').collapse('hide')

  Chart.defaults.global.plugins!.colorschemes.scheme = colorSelector.value

  ui.newChart('projectTotalsChart', projectTotalsChartConfig(projects));
  ui.newChart('activityChart', activityChartConfig(projects, daily));

  colorSelector.whenChange((select: HTMLSelectElement) => {
    ui.charts.forEach(chart => {
      chart.options.plugins!.colorschemes.scheme = select.value;
      chart.update();
    })
  })

});

import 'bootstrap'
import 'bootstrap-select'
