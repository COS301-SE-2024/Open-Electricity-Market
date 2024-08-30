<script>

  import { onMount, onDestroy } from 'svelte';
  import { API_URL_GRID, API_URL_MARKET } from '$lib/config.js';


  onMount(async () => {

    fetchData();
    interval = setInterval(fetchData, 11000);


    return () => {
      clearInterval(interval);
    };


  });

  $: consumers = null; 
  $: producers = null;
  $: generation = null;
  $: impedance = null;
  $: totalusers = null;
  const today = new Date();
  const options = { day: 'numeric', month: 'long', year: 'numeric' };
  const currdate = today.toLocaleDateString('en-US', options);


  async function fetchData() {
    try {
      const response = await fetch(`${API_URL_GRID}/stats`, {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json'
        }
      });
      
      const fdata = await response.json();
      console.log("Fetched statistics:", fdata);
      consumers = fdata.consumer_count; 
      producers = fdata.producer_count; 
      generation = fdata.total_generation; 
      impedance = fdata.total_impedance; 
      totalusers = fdata.user_count; 
      
      
      
    } catch (error) {
      console.log("There was an error fetching the JSON for the stats on grid sim:", error);
    }
  }

  // not sure why but this doesn't work
  // function formatNumber(number) {
  //   if (number > 1000) {
  //     number = (number/1000).toFixed(3);
  //     out = Intl.NumberFormat().format(number) + " k"
  //     return out;
  //   } else if (number > 1000000) {
  //     number = (number/1000000).toFixed(3);
  //     out = Intl.NumberFormat().format(number) + " M"
  //     return out;
  //   }
  // }

</script>

<div class="stats stats-vertical lg:stats-horizontal shadow w-full">
  <div class="stat">
    <div class="stat-title">Total Users</div>
    {#if totalusers == null}
    <span class="loading loading-spinner loading-lg"></span>
    {:else}
    <div class="stat-value font-normal">{Intl.NumberFormat().format(totalusers)}</div>
    {/if}
    <div class="stat-desc">{currdate}</div>
  </div>

  <div class="stat">
    <div class="stat-title">Number of producers</div>
    {#if producers == null}
    <span class="loading loading-spinner loading-lg"></span>  
    {:else}
    <div class="stat-value font-normal">{Intl.NumberFormat().format(producers)}</div>
    {/if}
    <!-- <div class="stat-desc">↗︎{(Math.random(80)*100).toFixed(2)}%</div> -->
  </div>

  <div class="stat">
    <div class="stat-title">Number of consumers</div>
    {#if consumers == null}
    <span class="loading loading-spinner loading-lg"></span>
    {:else}
    <div class="stat-value font-normal">{Intl.NumberFormat().format(consumers)}</div>
    {/if}
  </div>

   <div class="stat">
    <div class="stat-title">Total Generation</div>
    {#if generation == null}
    <span class="loading loading-spinner loading-lg"></span>  
    {:else}
    <div class="stat-value font-normal">{Intl.NumberFormat().format(generation)} V</div>
    {/if}
  </div>

  <div class="stat">
    <div class="stat-title">Total Impedance</div>
    {#if impedance == null}
    <span class="loading loading-spinner loading-lg"></span>
    {:else}
    <div class="stat-value font-normal">{Intl.NumberFormat().format(impedance)} Ω</div>
    {/if}
    <div class="stat-desc">Measure of opposition to electrical flow</div>
  </div>

</div>