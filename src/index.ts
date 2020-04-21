import { Commit, ProjectMap, getProjectMap, DailyHours, getDaily } from "./gtm";
import { DropdownSelect, UI, getCommitElement } from "./components";
import { Chart } from "chart.js"
import moment from 'moment';
import $ from 'jquery';
import 'bootstrap'
import 'chartjs-plugin-colorschemes';
import 'chartjs-plugin-zoom';
import 'daterangepicker'
import { projectTotalsChartConfig, activityChartConfig } from "./charts";

const ui = new UI()

    $(function() {

var start = moment().subtract(29, 'days');
var end = moment();

function cb(start:any, end:any) {
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
  const commitcounts: number[] = []
  const ds = []
  for (const pname in projects) {
    const p = projects[pname]
    labels.push(pname)
    commitcounts.push(p.commitcount)
    ds.push({
      data: [p.total],
      label: pname,
    })
  }

  const e = document.getElementById('commitsPlaceholder')
  for (const c of res.sort((c, d) => c.When >= d.When ? 1 : -1)) {
    if (c.Note.Files.length == 0) continue
    e!.insertAdjacentHTML('afterend', getCommitElement(c))
  }
  $('.collapse').collapse('hide')

  Chart.defaults.global.plugins!.colorschemes.scheme = colorSelector.value

  ui.newChart('projectTotalsChart', projectTotalsChartConfig(ds, commitcounts));
  ui.newChart('activityChart', activityChartConfig(projects, daily));

  colorSelector.whenChange((select: HTMLSelectElement) => {
    ui.charts.forEach(chart => {
      chart.options.plugins!.colorschemes.scheme = select.value;
      chart.update();
    })
  })

});
