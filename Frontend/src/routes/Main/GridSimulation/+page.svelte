

<script>
  import { onMount } from "svelte";

  let data = {};
  let interval; 

  onMount(async () => {
    await fetchData(); 
    interval = setInterval(fetchData, 800); 

   
    return () => {
      clearInterval(interval);
    };
  });

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

  async function fetchData() {
    try {
      const response = await fetch("http://localhost:8000/info", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        }
      });
      console.log("Request being sent...");
      const fdata = await response.json();
      console.log("Fetched data:", fdata);
      data = {
        ...fdata,
        Consumers: fdata.Consumers.map(item => JSON.parse(item)),
        Generators: fdata.Generators.map(item => JSON.parse(item)),
        Transformers: fdata.Transformers.map(item => JSON.parse(item)),
        "Transmission Lines": fdata["Transmission Lines"].map(item => JSON.parse(item))
      };
    } catch (error) {
      console.log("There was an error fetching the JSON for the overview:", error);
    }
  }
</script>

<main class="container mx-auto p-4">

   
 
  {#if Object.keys(data).length > 0}
    
    <section class="mb-8">
      <h2 class="text-2xl font-bold mb-4">Consumers</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each data.Consumers as consumer}
          <div class="card shadow-md p-4 transform transition duration-200 hover:scale-105 hover:shadow-lg">
            <h3 class="text-lg font-semibold mb-2">Consumer ID: {consumer.ID}</h3>
            <p><strong>Connected Transmission Line:</strong> {consumer["Connected Transmission Line"]}</p>
            <p><strong>Resistance:</strong> {consumer.Resistance} Ω</p>
            <p><strong>Phase 1 Voltage:</strong> {consumer.Voltage["Phase 1"]} V</p>
            <p><strong>Phase 2 Voltage:</strong> {consumer.Voltage["Phase 2"]} V</p>
            <p><strong>Phase 3 Voltage:</strong> {consumer.Voltage["Phase 3"]} V</p>
          </div>
        {/each}
      </div>
    </section>

    
    <section class="mb-8">
      <h2 class="text-2xl font-bold mb-4">Generators</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each data.Generators as generator}
          <div class="card shadow-md p-4 transform transition duration-300 hover:scale-105 hover:shadow-lg ">
            <h3 class="text-lg font-semibold mb-2">Generator ID: {generator.ID}</h3>
            <p><strong>Connected Transmission Line:</strong> {generator["Connected Transmission Line"]}</p>
            <p><strong>Frequency:</strong> {generator.Frequency} Hz</p>
            <p><strong>Max Voltage:</strong> {generator["Max Voltage"]} V</p>
            <p><strong>Phase 1 Voltage:</strong> {generator.Voltage["Phase 1"]} V</p>
            <p><strong>Phase 2 Voltage:</strong> {generator.Voltage["Phase 2"]} V</p>
            <p><strong>Phase 3 Voltage:</strong> {generator.Voltage["Phase 3"]} V</p>
          </div>
        {/each}
      </div>
    </section>

  
    <section class="mb-8">
      <h2 class="text-2xl font-bold mb-4">Transformers</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each data.Transformers as transformer}
          <div class="card shadow-md p-4 transform transition duration-300 hover:scale-105 hover:shadow-lg">
            <h3 class="text-lg font-semibold mb-2">Transformer ID: {transformer.ID}</h3>
            <p><strong>Primary Transmission Line:</strong> {transformer["Primary Transmission Line"]}</p>
            <p><strong>Secondary Transmission Line:</strong> {transformer["Secondary Transmission Line"]}</p>
            <p><strong>Ratio:</strong> {transformer.Ratio}</p>
          </div>
        {/each}
      </div>
    </section>

   
    <section class="mb-8">
      <h2 class="text-2xl font-bold mb-4">Transmission Lines</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each data["Transmission Lines"] as line}
          <div class="card shadow-md p-4 transform transition duration-300 hover:scale-105 hover:shadow-lg ">
            <h3 class="text-lg font-semibold mb-2">Transmission Line ID: {line.ID}</h3>
            <p><strong>Impedance:</strong> {line.Impedance} Ω</p>
            <p><strong>Resistance:</strong> {line.Resistance} Ω</p>
            <p><strong>Phase 1 Voltage:</strong> {line.Voltage["Phase 1"]} V</p>
            <p><strong>Phase 2 Voltage:</strong> {line.Voltage["Phase 2"]} V</p>
            <p><strong>Phase 3 Voltage:</strong> {line.Voltage["Phase 3"]} V</p>
          </div>
        {/each}
      </div>
    </section>
  {:else}
   <span class="loading loading-ring loading-lg ml-6"></span>
  {/if}
</main>

<style>
  
</style>
