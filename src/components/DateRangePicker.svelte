<script>
  import { onMount, createEventDispatcher } from "svelte";
  import moment from "moment";
  import JQ from "jquery";
  import "daterangepicker";

  const dispatch = createEventDispatcher();

  let reportrange;
  let datetext;

  onMount(() => {
    var start = moment().subtract(29, "days");
    var end = moment();

    function cb(start, end) {
      JQ(datetext).html(
        start.format("MMMM D, YYYY") + " - " + end.format("MMMM D, YYYY")
      );
      console.log("asdf");
      // fetchCommits(start.format("YYYY-MM-DD"), end.format("YYYY-MM-DD"));
      dispatch("change", {
        start: start.format("YYYY-MM-DD"),
        end: end.format("YYYY-MM-DD")
      });
    }

    JQ(reportrange).daterangepicker(
      {
        startDate: start,
        endDate: end,
        ranges: {
          Today: [moment(), moment()],
          Yesterday: [
            moment().subtract(1, "days"),
            moment().subtract(1, "days")
          ],
          "Last 7 Days": [moment().subtract(6, "days"), moment()],
          "Last 30 Days": [moment().subtract(29, "days"), moment()],
          "This Month": [moment().startOf("month"), moment().endOf("month")],
          "Last Month": [
            moment()
              .subtract(1, "month")
              .startOf("month"),
            moment()
              .subtract(1, "month")
              .endOf("month")
          ]
        }
      },
      cb
    );

    cb(start, end);
  });
</script>

<span
  bind:this={reportrange}
  class="navbar-text mr-sm-2"
  style="cursor: pointer">
  <i class="fa fa-calendar" />
  &nbsp;
  <span bind:this={datetext} />
  <i class="fa fa-caret-down" />
</span>
