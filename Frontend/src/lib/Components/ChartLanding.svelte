
<script>
  import { onMount, onDestroy } from 'svelte';
  import { createChart, sampleChartConfig } from './ChartLanding.js';
  

  let chart;
  let chartCanvas;
  //export let data = {}; 
  let interval; 

  onMount(() => {
    
    if (typeof window !== 'undefined') { // Check if running in the browser
      chart = createChart(chartCanvas, sampleChartConfig);
      // fetchData(); 
      //updateChart(data);  
      // interval = setInterval(updateChart, 1500);
    }

    return () => {
      if (chart) {
        chart.destroy();
      }
      // clearInterval(interval);
    };
  });

  function updateChart() {
    chart.data.datasets[0].data.push(Math.random(20));
    chart.data.datasets[1].data.push(Math.random(20));
    chart.data.datasets[2].data.push(Math.random(20));
    chart.data.labels.push(chart.data.labels.length + 1);

    chart.update();  
  }
</script>

<div style="display: flex; max-width: 100%;"> 
  <canvas bind:this={chartCanvas} height=280></canvas>
</div>

<style>
  /* canvas {
    max-width: 100%;
    max-height: 100%;
  } */
</style>
  