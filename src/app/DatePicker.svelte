<script>
  import { onMount, createEventDispatcher } from "svelte";
  import moment from "moment";

  const ranges = {
    Today: [moment(), moment()],
    Yesterday: [moment().subtract(1, "days"), moment().subtract(1, "days")],
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
    ],
    "This Year": [moment().startOf("year"), moment().endOf("year")],
    "Last Year": [
      moment()
        .subtract(1, "year")
        .startOf("year"),
      moment()
        .subtract(1, "year")
        .endOf("year")
    ],
    "Beginning of Time": [moment(new Date(1970, 1, 1)), moment()]
  };

  const dispatch = createEventDispatcher();

  const MONTH_NAMES = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December"
  ];
  const DAYS = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

  let datepickerValue = "";
  let month = "";
  let showDatepicker = false;
  let year = "";
  let no_of_days = [];
  let blankdays = [];

  function isToday(date) {
    const today = new Date();
    const d = new Date(year, month, date);
    return today.toDateString() === d.toDateString();
  }

  function selectRange(text, [start, end]) {
    datepickerValue = text;
    showDatepicker = false;

    dispatch("change", {
      start: start.format("YYYY-MM-DD"),
      end: end.format("YYYY-MM-DD")
    });
  }

  function getDateValue(day) {
    const selectedDate = new Date(year, month, day);
    datepickerValue = selectedDate.toDateString();
    showDatepicker = false;

    const start = moment(selectedDate);
    const end = moment();

    dispatch("change", {
      start: start.format("YYYY-MM-DD"),
      end: end.format("YYYY-MM-DD")
    });
  }

  function nextMonth() {
    if (month == 11) {
      month = 0;
      year++;
    } else {
      month++;
    }
  }

  function prevMonth() {
    if (month == 0) {
      month = 11;
      year--;
    } else {
      month--;
    }
  }

  function getNoOfDays() {
    const daysInMonth = new Date(year, month + 1, 0).getDate();

    // find where to start calendar day of week
    const dayOfWeek = new Date(year, month).getDay();
    const blankdaysArray = [];
    for (let i = 1; i <= dayOfWeek; i++) {
      blankdaysArray.push(i);
    }

    const daysArray = [];
    for (let i = 1; i <= daysInMonth; i++) {
      daysArray.push(i);
    }

    blankdays = blankdaysArray;
    no_of_days = daysArray;
  }

  onMount(() => {
    const today = new Date();
    month = today.getMonth();
    year = today.getFullYear();
    datepickerValue = new Date(year, month, today.getDate()).toDateString();

    getNoOfDays();

    const defaultRange = "Last 7 Days";
    selectRange(defaultRange, ranges[defaultRange]);
  });
</script>

<style>
  .modal-background {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.2);
  }
</style>

<div class="relative">

  <div class="bg-white flex items-center rounded-sm shadow-xl m-2">
    <input
      type="text"
      readonly
      value={datepickerValue}
      on:click={() => (showDatepicker = !showDatepicker)}
      on:keydown={e => {
        if (e.keyCode === 27) showDatepicker = false;
      }}
      class="px-3 py-0 leading-none focus:outline-none text-gray-700
      cursor-pointer"
      placeholder="Select date" />

    <svg
      class="my-1 mx-2 h-6 w-6 text-blue-700 cursor-pointer"
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      on:click={() => (showDatepicker = !showDatepicker)}>
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2
        2v12a2 2 0 002 2z" />
    </svg>
  </div>

  {#if showDatepicker}
    <div class="modal-background" on:click={() => (showDatepicker = false)} />

    <div
      class="bg-white mt-12 rounded-lg shadow p-4 absolute top-0 left-0"
      style="width: 25rem">

      <div class="flex divide-x">
        <div style="width: 9rem">
          {#each Object.entries(ranges) as [text, range]}
            <button
              class="block text-gray-800 focus:outline-none hover:bg-blue-200
              rounded pt-1 px-2"
              type="button"
              on:click={() => selectRange(text, range)}>
              {text}
            </button>
          {/each}

        </div>
        <div class="flex-1 pl-5">
          <div class="flex justify-between items-center mb-2">
            <div>
              <span class="text-lg font-bold text-gray-800">
                {MONTH_NAMES[month]}
              </span>
              <span class="ml-1 text-lg text-gray-600 font-normal">{year}</span>
            </div>
            <div>
              <button
                type="button"
                class="transition ease-in-out duration-100 inline-flex
                cursor-pointer hover:bg-gray-200 focus:outline-none p-1
                rounded-full"
                on:click={() => {
                  prevMonth();
                  getNoOfDays();
                }}>
                <svg
                  class="h-6 w-6 text-gray-500 inline-flex"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M15 19l-7-7 7-7" />
                </svg>
              </button>
              <button
                type="button"
                class="transition ease-in-out duration-100 inline-flex
                cursor-pointer hover:bg-gray-200 focus:outline-none p-1
                rounded-full"
                on:click={() => {
                  nextMonth();
                  getNoOfDays();
                }}>
                <svg
                  class="h-6 w-6 text-gray-500 inline-flex"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M9 5l7 7-7 7" />
                </svg>
              </button>
            </div>
          </div>

          <div class="flex flex-wrap mb-3 -mx-1">
            {#each DAYS as day}
              <div style="width: 14.26%" class="px-1">
                <div class="text-gray-800 font-medium text-center text-xs">
                  {day}
                </div>
              </div>
            {/each}
          </div>

          <div class="flex flex-wrap -mx-1">
            {#each blankdays as blankday}
              <div
                style="width: 14.28%"
                class="text-center border p-1 border-transparent text-sm" />
            {/each}
            {#each no_of_days as day}
              <div style="width: 14.28%" class="px-1 mb-1">
                <div
                  on:click={() => getDateValue(day)}
                  class="cursor-pointer text-center text-sm leading-none
                  rounded-full leading-loose transition ease-in-out duration-100
                  {isToday(day) ? 'bg-blue-500 text-white' : 'text-gray-700 hover:bg-blue-200'}">
                  {day}
                </div>
              </div>
            {/each}
          </div>

        </div>
      </div>
    </div>
  {/if}
</div>
