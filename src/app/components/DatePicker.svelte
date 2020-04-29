<script>
  import { onMount } from "svelte";

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
  let container;

  function isToday(date) {
    const today = new Date();
    const d = new Date(year, month, date);
    return today.toDateString() === d.toDateString();
  }

  function getDateValue(day) {
    const selectedDate = new Date(year, month, day);
    datepickerValue = selectedDate.toDateString();
    showDatepicker = false;
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

  function isOutside(target) {
    let parent = target;
    while (parent) {
      if (parent === container) {
        return false;
      }
      parent = parent.parentNode;
    }
    return true;
  }

  onMount(() => {
    const today = new Date();
    month = today.getMonth();
    year = today.getFullYear();
    datepickerValue = new Date(year, month, today.getDate()).toDateString();

    getNoOfDays();
  });
</script>

<svelte:body
  on:click={e => {
    if (isOutside(e.target)) showDatepicker = false;
  }} />

<div bind:this={container} class="relative">
  <input
    type="text"
    readonly
    value={datepickerValue}
    on:click={() => (showDatepicker = !showDatepicker)}
    on:keydown={e => {
      if (e.keyCode === 27) showDatepicker = false;
    }}
    class="pl-4 pr-10 py-3 leading-none rounded-lg shadow-sm focus:outline-none
    focus:shadow-outline text-gray-600 font-medium"
    placeholder="Select date" />

  <div class="absolute top-0 right-0 px-3 py-2">
    <svg
      class="h-6 w-6 text-gray-400"
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
    <div
      class="bg-white mt-12 rounded-lg shadow p-4 absolute top-0 left-0"
      style="width: 17rem"
      click.away="showDatepicker = false">

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
            cursor-pointer hover:bg-gray-200 focus:outline-none p-1 rounded-full"
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
            cursor-pointer hover:bg-gray-200 focus:outline-none p-1 rounded-full"
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
              rounded-full leading-loose transition ease-in-out duration-100 {isToday(day) ? 'bg-blue-500 text-white' : 'text-gray-700 hover:bg-blue-200'}">
              {day}
            </div>
          </div>
        {/each}
      </div>

    </div>
  {/if}
</div>
