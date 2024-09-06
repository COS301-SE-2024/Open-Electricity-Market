<script>
  import { onMount, onDestroy } from "svelte";
  import { createChart, sampleChartConfig } from "./Chart.js";

  let chart;
  let chartCanvas;
  export let data = [];
  let interval;

  function resizeChart() {
    if (window.innerWidth <= 760) {
      chartCanvas.style.height = "320px";
    } else {
      chartCanvas.style.height = "700px";
    }
  }

  // onMount(() => {
  //   resizeChart();
  //   window.addEventListener('resize', resizeChart);
  //   return () => window.removeEventListener('resize', resizeChart);
  // });

  onMount(async () => {
    if (typeof window !== "undefined") {
      // Check if running in the browser
      chart = createChart(chartCanvas, sampleChartConfig);
      fillChart();
    }
    // resizeChart();
    window.addEventListener("resize", resizeChart);
    // interval = setInterval(resizeChart, 5000);

    return () => {
      if (chart) {
        chart.destroy();
      }
      window.removeEventListener("resize", resizeChart);
    };
  });

  $: if (chart && data.length > 0) {
    console.log("data was updated..." + data);
    // updateChart();
    updateChart();
    //  console.log("this was a succcess");
  }

  //  $: if (chart ) {
  //   updateChart();
  // }

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

      // chart.data.datasets[1].data.push(data.Phase2);
      // chart.data.datasets[2].data.push(data.Phase3);
      // chart.data.labels.push(chart.data.labels.length + 1);
      //chart.update();
    }
    return;
  }
</script>

<div style="display: flex; max-height: 100%">
  <canvas bind:this={chartCanvas} height="340px"></canvas>
</div>

<style>
  /* canvas {
    max-width: 100%;
    max-height: 100%;
  } */
</style>
