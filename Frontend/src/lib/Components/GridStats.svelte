<script>

  import { onMount, onDestroy } from 'svelte';

  onMount(async () => {

    fetchData();
    interval = setInterval(fetchData, 11000);



  });

  let consumers; 
  let producers; 
  let generation; 
  let impedance; 
  let totalusers; 
  const today = new Date();
  const options = { day: 'numeric', month: 'long', year: 'numeric' };
  const currdate = today.toLocaleDateString('en-US', options);


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
      consumers = fdata.consumer_count; 
      producers = fdata.producer_count; 
      generation = fdata.total_generation; 
      impedance = fdata.total_impedance; 
      totalusers = fdata.user_count; 
      
      
      
    } catch (error) {
      console.log("There was an error fetching the JSON for the stats on grid sim:", error);
    }
  }


</script>

<div class="stats stats-vertical lg:stats-horizontal shadow w-full">
  <div class="stat">
    <div class="stat-title">Total Users</div>
    <div class="stat-value">{Intl.NumberFormat().format(totalusers)}</div>
    <div class="stat-desc">{currdate}</div>
  </div>

  <div class="stat">
    <div class="stat-title">Number of producers</div>
    <div class="stat-value">{Intl.NumberFormat().format(producers)}</div>
    <!-- <div class="stat-desc">↗︎{(Math.random(80)*100).toFixed(2)}%</div> -->
  </div>

  <div class="stat">
    <div class="stat-title">Number of consumers</div>
    <div class="stat-value">{Intl.NumberFormat().format(consumers)}</div>
  </div>

   <div class="stat">
    <div class="stat-title">Total Generation</div>
    <div class="stat-value">{Intl.NumberFormat().format(generation)} V</div>
  </div>

  <div class="stat">
    <div class="stat-title">Total Impedance</div>
    <div class="stat-value">{Intl.NumberFormat().format(impedance)} Ω</div>
    <div class="stat-desc">Measure of opposition to electrical flow</div>
  </div>
</div>