
<script>
  import { onMount, onDestroy } from 'svelte';
  import { createChart, sampleChartConfig } from './Chart2.js';

  let chart;
  let chartCanvas;
  export let data = {}; 
  let interval; 

  onMount(async () => {
    
    if (typeof window !== 'undefined') { // Check if running in the browser
      chart = createChart(chartCanvas, sampleChartConfig);
      // fetchData(); 
      updateChart(data);
      // interval = setInterval(updateChart, 1000);
    }

    return () => {
      if (chart) {
        chart.destroy();
      }
      // clearInterval(interval);
    };
  });

  

  // async function fetchData() {

  //     try {
  //       const response = await fetch("http://localhost:8000/overview", {
  //     method: "POST", 
  //     headers: {
  //       'Content-Type': 'application/json' 
  //     }

  //   });
  //       console.log("request being sent...");
  //       // const response = fetch("http://localhost:8000");
  //       const data = await response.json();
  //       console.log(data);
  //       //Voltage 1,2,3 as well as price
  //       updateChart(data.Phase1, data.Phase2);
  //     } catch (error) {
  //       console.log("There was an error fetching the JSON for the chart..", error);
  //     }
  // };

    $: if (chart && data.Consumers) {
    console.log("Reactive if was triggered...");
    updateChart(data);
  }

  //  $: if (chart ) {
  //   updateChart();
  // }

  async function updateChart(data){

    //this will have to check for price once endpoint changes *************
    if(chart && data.Consumers){     
      console.log("Update chart with new data...");
      const consumer = data.Consumers[0];
      chart.data.datasets[0].data.push(consumer.Voltage["Phase 1"]);
    //   console.log(consumer.Voltage["Phase 1"]);
      chart.data.datasets[1].data.push(consumer.Voltage["Phase 2"]);
      chart.data.datasets[2].data.push(consumer.Voltage["Phase 3"]);
      chart.data.labels.push(chart.data.labels.length + 1); 
      chart.update();
    }
    else{
        console.log("no update occurring");
    }
      return; 

  }



 

</script>

<div style="display: flex; max-width: 50%;"> 
  <canvas bind:this={chartCanvas} height=240></canvas>
</div>

<style>
  /* canvas {
    max-width: 100%;
    max-height: 100%;
  } */
</style>
  