
<script>
  import { onMount, onDestroy } from 'svelte';
  import { createChart, sampleChartConfig } from './Chart.js';

  let chart;
  let chartCanvas;
  let data; 
  let interval; 

  onMount(() => {
    if (typeof window !== 'undefined') { // Check if running in the browser
      chart = createChart(chartCanvas, sampleChartConfig);
      fetchData(); 
      interval = setInterval(fetchData, 1000);
    }

    return () => {
      if (chart) {
        //chart.destroy();
      }
      clearInterval(interval);
    };
  });

  

  async function fetchData() {

      try {
        const response = await fetch("http://localhost:8000/overview", {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json' 
      }
    });
        console.log("request being sent...");
        // const response = fetch("http://localhost:8000");
        const data = await response.json();
        console.log(data);
        //Voltage 1,2,3 as well as price
        updateChart(data.Voltage1, data.Voltage2);
      } catch (error) {
        console.log("There was an error fetching the JSON for the chart..", error);
      }
  };

  function updateChart(data1, data2){

    if(chart){
      chart.data.datasets[0].data.push(data1);
      chart.data.datasets[1].data.push(data2);
      chart.data.labels.push(chart.data.labels.length + 1); 
      chart.update();
    }
      

  }



 

</script>

<div style="display: flex; max-width: 50%;"> 
  <canvas bind:this={chartCanvas} height="300"></canvas>
</div>

<style>
  /* canvas {
    max-width: 100%;
    max-height: 100%;
  } */
</style>
  