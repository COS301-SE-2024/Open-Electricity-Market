
<script>
  import { onMount, onDestroy } from 'svelte';
  import { createChart, sampleChartConfig } from './Chart.js';

  let chart;
  let chartCanvas;
  export let data = {}; 
  let interval; 

  onMount(async () => {
    
    if (typeof window !== 'undefined') { // Check if running in the browser
      chart = createChart(chartCanvas, sampleChartConfig);
      
      
    }

    return () => {
      if (chart) {
        chart.destroy();
      }
     
    };
  });

  

  

    $: if (chart && Object.keys(data).length) {
      // console.log("Reactive if was triggered...");
      updateChart();
  }

  //  $: if (chart ) {
  //   updateChart();
  // }

  async function updateChart(){

    //this will have to check for price once endpoint changes *************
    if(chart && data){
      // console.log("This is data: ", data);
      // console.log("UPDate chart is reactive on chart js.....");
      if (chart.data.datasets[0].data.length > 21) {
        chart.data.datasets[0].data.shift();
        // console.log(chart.data.datasets[0].data)
      }
      chart.data.datasets[0].data.push(data.price);

      // chart.data.datasets[1].data.push(data.Phase2);
      // chart.data.datasets[2].data.push(data.Phase3);
      // chart.data.labels.push(chart.data.labels.length + 1);
      chart.update();
    }
      return; 

  }



 

</script>

<div style="display: flex;max-width: 80%;">
  <canvas bind:this={chartCanvas} height=240></canvas>
</div>

<style>
  /* canvas {
    max-width: 100%;
    max-height: 100%;
  } */
</style>
  