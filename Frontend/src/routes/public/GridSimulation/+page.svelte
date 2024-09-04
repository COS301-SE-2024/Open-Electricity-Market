<script>
  import { onMount } from "svelte";
  import Map from "$lib/Components/Map.svelte";
  import Chart from "$lib/Components/Chart2.svelte";
  import GridStats from "../../../lib/Components/GridStats.svelte";
  import { API_URL_GRID, API_URL_MARKET } from "$lib/config.js";

  let data = {};
  let interval;
  // let dropdownViewable = false;
  let mapdata;

  // function toggleDropdown() {
  //   dropdownViewable = !dropdownViewable;
  // }

  onMount(async () => {
    await fetchData();
    await fetchstart();
    interval = setInterval(fetchData, 10000);

    return () => {
      clearInterval(interval);
      Map.destroy();
    };
  });

  async function fetchstart() {
    try {
      const response = await fetch(`${API_URL_GRID}/start`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
        },
      });
      console.log("start being sent...");
      // const response = fetch("http://localhost:8000");
      const startdata = await response.json();
      console.log(startdata);
      //Voltage 1,2,3 as well as price
      //updateChart(data.Phase1, data.Phase2);
    } catch (error) {
      console.log(
        "There was an error fetching the JSON for the chart..",
        error
      );
    }
  }

  async function fetchData() {
    try {
      const response = await fetch(`${API_URL_GRID}/info`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
        },
      });
      //console.log("Request being sent...");
      const fdata = await response.json();
      console.log("Fetched data4:", fdata);
      mapdata = fdata.circuits[0];
    } catch (error) {
      console.log(
        "There was an error fetching the JSON for the overview:",
        error
      );
    }
  }

  function handleMarkerClick(entity) {
    // console.log(entity.detail.voltage);
    // data = entity.detail;
    const markerData = entity.detail;
    data = { ...markerData.voltage };
    console.log("Updated data is this: ", data);
  }
</script>

<main class="container sm:mx-auto">
  <div class="fullsection flex xs:flex-row -mt-6 w-full">
    <div class="bg-base-100 mx-2 p-4 rounded-2xl h-4/6 w-2/3">
      <div class="mapsection md:w-full xs:w-full">
        <Map {mapdata} on:markerClick={handleMarkerClick} />
      </div>

      <div class="statsection my-2 bottom-0">
        <GridStats />
      </div>
    </div>
    <div
      class="chartsection md:w-1/4 mx-2 p-5 xs:w-full bg-base-100 rounded-2xl"
    >
      <Chart {data} />
    </div>
  </div>
  <div class="w-full py-20 flex justify-center">
    <div class="card glass bg-base-100 p-6 w-2/3">
      <h1 class="text-2xl">This is a paragraph</h1>
      <p>
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
        veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
        commodo consequat. Duis aute irure dolor in reprehenderit in voluptate
        velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint
        occaecat cupidatat non proident, sunt in culpa qui officia deserunt
        mollit anim id est laborum.
      </p>
    </div>
  </div>
</main>
