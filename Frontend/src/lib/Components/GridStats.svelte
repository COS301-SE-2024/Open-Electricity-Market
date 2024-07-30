<script>

  import { onMount, onDestroy } from 'svelte';

  onMount(async () => {

    fetchData();
    interval = setInterval(fetchData, 1000);



  });


   async function fetchData() {
    try {
      const response = await fetch("http://localhost:8000/stats", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        }
      });
      
      const fdata = await response.json();
      console.log("Fetched statistics:", fdata);
      
      
    } catch (error) {
      console.log("There was an error fetching the JSON for the stats on grid sim:", error);
    }
  }


</script>

<div class="stats stats-vertical lg:stats-horizontal shadow">
  <div class="stat">
    <div class="stat-title">Downloads</div>
    <div class="stat-value">31K</div>
    <div class="stat-desc">Jan 1st - Feb 1st</div>
  </div>

  <div class="stat">
    <div class="stat-title">New Users</div>
    <div class="stat-value">4,200</div>
    <div class="stat-desc">↗︎ 400 (22%)</div>
  </div>

  <div class="stat">
    <div class="stat-title">New Registers</div>
    <div class="stat-value">1,200</div>
    <div class="stat-desc">↘︎ 90 (14%)</div>
  </div>
</div>