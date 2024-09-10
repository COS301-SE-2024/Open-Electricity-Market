<script>
  import { onMount } from "svelte";
  import Map from "$lib/Components/Map.svelte";
  import Chart from "$lib/Components/Chart2.svelte";
  import GridStats from "../../../lib/Components/GridStats.svelte";
  import { API_URL_GRID, API_URL_MARKET } from "$lib/config.js";

  let data = {};
  let interval;
  let numDecimals = 2;
  let advancedView = false;
  let dropdownViewable = false;
  let mapdata;
  let oscilloscopedata = null;

  function toggleDropdown() {
    dropdownViewable = !dropdownViewable;
  }

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
        'Content-Type': 'application/json',
        'Accept': 'application/json'
      }

    });
        console.log("start being sent...");
        // const response = fetch("http://localhost:8000");
        const startdata = await response.json();
        console.log(startdata);
        //Voltage 1,2,3 as well as price
        //updateChart(data.Phase1, data.Phase2);
      } catch (error) {
        console.log("There was an error fetching the JSON for the chart..", error);
        
      }
  };

   function setAdvancedView(){
    advancedView = !advancedView;
    if (advancedView) {
      numDecimals = 7;
    } else {
      numDecimals = 2;
    }
  }

  async function fetchData() {
    try {
      const response = await fetch(`${API_URL_GRID}/info`, {
        method: "POST",
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json'
        }
      });
      //console.log("Request being sent...");
      const fdata = await response.json();
      console.log("Fetched data:", fdata);
      mapdata = fdata.circuits[0];
      // data = null;
      // data = {
      //   ...fdata,
      //   Consumers: fdata.Consumers.map(item => JSON.parse(item)),
      //   Generators: fdata.Generators.map(item => JSON.parse(item)),
      //   Transformers: fdata.Transformers.map(item => JSON.parse(item)),
      //   "Transmission Lines": fdata["Transmission Lines"].map(item => JSON.parse(item))
      // };
      // console.log("Data is this: ");
      // console.log(data);
      // chartdata = data[Consumers.Voltage.Phase1];
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
    console.log("Marker clicked:", markerData);
    data = { ...markerData.voltage };
    console.log("Updated data is this: ", data);
  }
</script>

<main class="container sm:mx-auto">

  <div class="fullsection flex xs:flex-col -mt-6">

    <div class="bg-base-100 p-4 rounded-2xl h-4/6">

      <div class="mapsection md:w-full xs:w-full">
        <Map {mapdata} on:markerClick = {handleMarkerClick} /> 
      </div>
  
      <div class="statsection my-2 bottom-0">
        <GridStats /> 
      </div>
      <div class="chartsection md:w-2/5 md:h-min p-5 xs:w-full bg-base-100 rounded-2xl hidden">   
        <Chart {data} />  
      </div>
    </div>


  </div>
</main>
