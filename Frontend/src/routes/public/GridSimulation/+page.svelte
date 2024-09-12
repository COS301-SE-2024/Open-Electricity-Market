<script>
  import { onMount } from "svelte";
  import Map from "$lib/Components/Map.svelte";
  import Chart from "$lib/Components/Chart2.svelte";
  import GridStats from "../../../lib/Components/GridStats.svelte";
  import { API_URL_GRID } from "$lib/config.js";

  $: voltageData = null;
  $: power = null;
  // let interval;
  // let dropdownViewable = false;
  $: mapdata = null;

  // function toggleDropdown() {
  //   dropdownViewable = !dropdownViewable;
  // }

  onMount(async () => {
    await fetchData();
    // interval = setInterval(fetchData, 10000);

    return () => {
      // clearInterval(interval);
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
      // console.log(startdata);
      //Voltage 1,2,3 as well as price
      //updateChart(data.Phase1, data.Phase2);
    } catch (error) {
      console.log("There was an error fetching the JSON for start()..", error);
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
      // console.log("Fetched data [gridsim /info]:", fdata);
      mapdata = fdata.circuits[0]; // TODO: multiple circuits exist
      if (!fdata.started) {
        await fetchstart();
      }
    } catch (error) {
      console.log(
        "There was an error fetching the JSON for the overview:",
        error
      );
    }
  }

  let markerDetails = null;
  function handleMarkerClick(entity) {
    // console.log(entity);
    markerDetails = entity.detail;
    voltageData = { ...markerDetails.voltage };

    power =
      (Math.pow(markerDetails.voltage.oscilloscope_detail.amplitude) ,
      2)/markerDetails.resistance;
    console.log(markerDetails.voltage.oscilloscope_detail.amplitude+" "+markerDetails.resistance+" "+power);
  }
</script>

<main class="container sm:mx-auto">
  <div class="fullsection flex xs:flex-row -mt-6 w-full justify-center">
    <div class="bg-base-100 mx-2 p-4 rounded-2xl h-4/6 w-2/3">
      <div class="mapsection md:w-full xs:w-full">
        {#if mapdata != null}
          <Map {mapdata} on:markerClick={handleMarkerClick} />
        {/if}
      </div>

      <div class="statsection mt-2">
        <GridStats />
      </div>
    </div>
    {#if voltageData != null}
      <div
        class="chartsection md:w-1/4 mx-2 p-5 xs:w-full bg-base-100 rounded-2xl flex flex-col max-w-full"
      >
        <h1 class="text-3xl">Consumer Details</h1>
        <hr />
        <span class="pt-5">
          <span class="font-light text-lg mt-10">Consumption: </span><br />
          <span class="text-4xl"
            >{Intl.NumberFormat().format(power)} W</span
          > <br />
          <span class="font-light text-lg mt-10">Impedance: </span><br />
          <span class="text-4xl"
            >{Intl.NumberFormat().format(
              (markerDetails.resistance / 1000).toFixed(3)
            )} kΩ</span
          >
        </span>
        {#if markerDetails.generators != null}
          <span class="pt-5">
            <span class="font-light text-lg">Generators: </span> <br />
            {#each markerDetails.generators as generator}
              <span class="text-2xl">Generator ID: {generator.id}</span><br />
              <span class="text-2xl"
                >Current Generation: {(
                  Math.pow(generator.max_voltage, 2) /
                  markerDetails.resistance /
                  1000
                ).toFixed(3)} kW</span
              >
            {/each}
          </span>
        {/if}
        <h1 class="font-light text-lg pt-5">Voltage and Phase</h1>
        <div class="h-max w-full">
          <Chart data={voltageData} />
        </div>
        <div class="flex w-full justify-end -mt-4">
          <span class="text-lg font-light"
            >{voltageData.oscilloscope_detail.frequency} Hz</span
          >
          <!-- <span class="text-lg font-light"
            >{voltageData.oscilloscope_detail.amplitude}</span
          > -->
        </div>
      </div>
    {:else}
      <div
        class="chartsection md:w-1/4 mx-2 p-5 xs:w-full bg-base-100 rounded-2xl"
      >
        <h1 class="text-4xl mb-6">Click on a marker to begin</h1>
        <p>Click on a marker to learn more about the activity on our grid!</p>
      </div>
    {/if}
  </div>
  <!-- <div class="w-full py-20 flex justify-center">
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
  </div> -->
</main>
