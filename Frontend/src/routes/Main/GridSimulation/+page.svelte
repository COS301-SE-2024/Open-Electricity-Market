<script>
    import Chart from '$lib/Components/Chart.svelte'


     async function fetchstart() {

      try {
        const response = await fetch("http://localhost:8000/start", {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json' 
      }

    });
        console.log("start being sent...");
        // const response = fetch("http://localhost:8000");
        const data = await response.json();
        console.log(data);
        //Voltage 1,2,3 as well as price
        //updateChart(data.Phase1, data.Phase2);
      } catch (error) {
        console.log("There was an error fetching the JSON for the chart..", error);
      }
  };

</script>

<main class="container mx-auto">
  <h1 class="text-2xl font-bold mb-4">Grid Simulation</h1>
  <p>Here you can simulate the national grid.</p>
  
  <button onclick={fetchstart()} class = "bg-red-500">start</button>

 <Chart class = "w-3/5 h-3/5"  />


  

 






</main>

<style>
  
</style>
