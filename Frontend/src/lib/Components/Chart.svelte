
<script>
  import { onMount, onDestroy } from 'svelte';
  import { createChart, sampleChartConfig } from './Chart.js';

  let chart;
  let chartCanvas;
  export let data = {}; 
  let interval; 
  

  function resizeChart() {
    if (window.innerWidth <= 760) {
      // chart.style.width = '100%';
      chartCanvas.style.height = '320px';
      // chartCanvas.style.width = '200px'; 
      console.log("If statement is running...");
    } else {
      chartCanvas.style.height = '700px';
      // chartCanvas.style.width = '900px'; 
      console.log("Else was executed...");
      // chart.style.height = '600px';
    }
  }

  // onMount(() => {
  //   resizeChart();
  //   window.addEventListener('resize', resizeChart);
  //   return () => window.removeEventListener('resize', resizeChart);
  // });

  onMount(async () => {
    
    if (typeof window !== 'undefined') { // Check if running in the browser
      chart = createChart(chartCanvas, sampleChartConfig);
    }
    resizeChart();
    window.addEventListener('resize', resizeChart);
    interval = setInterval(resizeChart, 5000);

    

    return () => {
      if (chart) {
        chart.destroy();
      }
      window.removeEventListener('resize', resizeChart);
     
    };
  });

  

  

    $: if (chart && Object.keys(data).length) {
      console.log("Reactive if was triggered...");
      updateChart();
  }

  //  $: if (chart ) {
  //   updateChart();
  // }

  async function updateChart(){

    //this will have to check for price once endpoint changes *************
    if(chart && data){
      console.log("This is data: ", data);
      console.log("UPDate chart is reactive on chart js.....");
      if (chart.data.datasets[0].data.length > 21) {
        chart.data.datasets[0].data.shift();
        console.log(chart.data.datasets[0].data)
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

<div style="display: flex; max-height: 100%" >
  <canvas bind:this={chartCanvas} height = 340px ></canvas>
</div>

<style>
  /* canvas {
    max-width: 100%;
    max-height: 100%;
  } */
</style>
  








