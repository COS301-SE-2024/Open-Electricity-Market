<script>
  import { onMount, onDestroy } from "svelte";
  import { createChart, sampleChartConfig } from "./PriceHistoryChart.js";

  let chart;
  let chartCanvas;
  export let data = [];

  function resizeChart() {
    if (window.innerWidth <= 760) {
      chartCanvas.style.height = "320px";
    } else {
      chartCanvas.style.height = "700px";
    }
  }

  onMount(async () => {
    if (typeof window !== "undefined") {
      // Check if running in the browser
      chart = createChart(chartCanvas, sampleChartConfig);
      fillChart();
    }
    // resizeChart();
    // window.addEventListener("resize", resizeChart);
    // interval = setInterval(resizeChart, 5000);

    return () => {
      if (chart) {
        chart.destroy();
      }
      // window.removeEventListener("resize", resizeChart);
    };
  });

  $: if (chart && data.length > 0) {
    updateChart();
  }

  function fillChart() {
    if (chart && data.length > 0) {
      chart.data.datasets[0].data = data;
      chart.data.labels = data.map((_, index) => index + 1);
      chart.update();
    }
  }

  async function updateChart() {
    if (chart && data.length > 0) {
      chart.data.datasets[0].data = data;
      chart.data.labels = data.map((_, index) => index + 1);
      chart.update();
    }
    return;
  }
</script>

<div class="h-full w-full">
  <canvas bind:this={chartCanvas}></canvas>
</div>

<style>
  /* canvas {
    max-width: 100%;
    max-height: 100%;
  } */
</style>
