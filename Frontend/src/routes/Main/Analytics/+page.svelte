<script>
import PieChart from "$lib/Components/PieChart.svelte";
import {onMount} from "svelte";
import { goto } from '$app/navigation';
import { API_URL_GRID, API_URL_MARKET, API_URL_AGENT } from '$lib/config.js';


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


