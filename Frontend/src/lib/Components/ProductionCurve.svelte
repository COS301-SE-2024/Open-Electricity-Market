<script>
  import { onMount, onDestroy } from "svelte";
  import { createChart, sampleChartConfig } from "./ProductionCurve.js";

  let chart;
  let chartCanvas;
  export let data = [];
  let interval;

  function resizeChart() {
    if (window.innerWidth <= 760) {
      chartCanvas.style.height = "150px";
      chartCanvas.style.width = "150px";
      chartCanvas.style.marginleft = "0rem"; 
    } else {
      chartCanvas.style.height = "700px";
    }
  }

  onMount(async () => {
    if (typeof window !== "undefined") {
      chart = createChart(chartCanvas, sampleChartConfig);
    }

    window.addEventListener("resize", resizeChart);

    return () => {
      if (chart) {
        chart.destroy();
      }
      window.removeEventListener("resize", resizeChart);
    };
  });

  $: if (chart && data.length > 0) {
    console.log("data was updated..." + data);
    updateChart();
  }

  async function updateChart() {
    if (chart && data.length > 0) {
      chart.data.datasets[0].data = data.map((value, _) => value / 1000);
      chart.data.labels = data.map((_, index) => index + 1);
      chart.update();
    }
    return;
  }
</script>

<div class="chartstyle">
  <canvas bind:this={chartCanvas} height="450px" width = "350px"></canvas>
</div>

<style>
  /* canvas {
    max-width: 100%;
    max-height: 100%;
  } */
  .chartstyle{
    display: flex; 
    max-height: 100%; 
    margin-left: 4rem; 
    @media (max-width:490px){
      margin-left: 0rem;
    }
  }
</style>
