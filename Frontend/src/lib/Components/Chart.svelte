<script>
  import { onMount, onDestroy } from "svelte";
  import { createChart, sampleChartConfig } from "./Chart.js";

  let chart;
  let chartCanvas;
  export let chartData;

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
    window.addEventListener("resize", resizeChart);

    return () => {
      if (chart) {
        chart.destroy();
      }
      window.removeEventListener("resize", resizeChart);
    };
  });

  $: if (chart && chartData != null) {
    updateChart();
  }

  function fillChart() {
    if (chart && chartData != null) {
      chart.data.datasets[0].data = chartData.prices;
      chart.data.labels = chartData.prices.map(
        (_, index) => new Date(chartData.timestamps[index])
      );
      chart.update();
    }
  }

  async function updateChart() {
    if (chart && chartData != null) {
      chart.data.datasets[0].data = chartData.prices;
      chart.data.labels = chartData.prices.map(
        (_, index) => new Date(chartData.timestamps[index])
      );
      chart.update();
    }
    return;
  }
</script>

<div style="display: flex; height: 100%; width: 100%">
  <canvas bind:this={chartCanvas}></canvas>
</div>

<style>
  /* canvas {
    max-width: 100%;
    max-height: 100%;
  } */
</style>
