
<script>
  import { onMount, onDestroy } from 'svelte';
  import { createChart, sampleChartConfig } from './ProductionCurve.js';

  let chart;
  let chartCanvas;
  export let data = []; 
  let interval; 
  

  function resizeChart() {
    if (window.innerWidth <= 760) {
      chartCanvas.style.height = '320px'; 
    } else {
      chartCanvas.style.height = '700px';
    }
  }

  onMount(async () => { 
     
    if (typeof window !== 'undefined') {
      chart = createChart(chartCanvas, sampleChartConfig);
      fillChart();
    }
    
    window.addEventListener('resize', resizeChart);

    return () => {
      if (chart) {
        chart.destroy();
      }
      window.removeEventListener('resize', resizeChart);
    };
  });

  

  

    $: if (chart && data.length>0) {
      console.log("data was updated..."+ data);
      updateChart();
  }

 

  

  async function updateChart(){
    
    if(chart && data.length>0){
  
      chart.data.datasets[0].data = data; 
      chart.data.labels = data.map((_, index) => index + 1); 
      chart.update(); 
    }
      return; 
  }


  



 

</script>

<div style="display: flex; max-height: 100%" >
  <canvas bind:this={chartCanvas} height = 340px ></canvas>
</div>

<style>
  /* canvas {
    max-width: 100%;
    max-height: 100%;
  } */
</style>
  








