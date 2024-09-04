<script>
import PieChart from "$lib/Components/PieChart.svelte";
import {onMount} from "svelte";
import { goto } from '$app/navigation';
import { API_URL_GRID, API_URL_MARKET, API_URL_AGENT } from '$lib/config.js';
import ConsumptionCurve from "$lib/Components/ConsumptionCurve.svelte";
  import ProductionCurve from "../../../lib/Components/ProductionCurve.svelte";


 async function fetchAgentData() {
    // try {
    //   const response = await fetch(`${API_URL_MARKET}/get_nodes`, {
    //     method: "POST", 
    //     headers: {
    //       'Content-Type': 'application/json',
    //       'Accept': 'application/json',
    //       'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
    //     },
    //     credentials: "include", 
    //     body: JSON.stringify({
    //       limit: 10
    //     })
    //   });
   
      
    //   const fdata = await response.json();
      
  
    // } catch (error) {
    //   console.log("An error occurred while fetching agent data..\n", error);
    // }
  };


</script>

<div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
    <PieChart />
</div>
<div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
   <ConsumptionCurve />
</div>
<div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
    <ProductionCurve />
</div>
<div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
    <PieChart />
</div>
<div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
    <span class = "font-light">Market Stats</span>
    <br>
    <span class = "font-light">Minimum price bought at: </span>
    <br>
    <span class = "font-light">Maximum price sold at: </span>
</div>


