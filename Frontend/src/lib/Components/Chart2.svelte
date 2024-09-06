<script>
  import { onMount, onDestroy } from "svelte";
  import { createChart, sampleChartConfig } from "./Chart2.js";

  export let data = {};

  let chart;
  let chartCanvas;
  let interval;
  const duration = 4;
  const sampleRate = 1000;
  const windowsize = 1;
  let initialized = false;

  let datasin = [];
  let datasin2 = [];
  let datasin3 = [];
  let labels = [];

  onMount(() => {
    if (typeof window !== "undefined") {
      // Check if running in the browser
      chart = createChart(chartCanvas, sampleChartConfig);
      // fetchData();
      //updateChart(data);
      if (!interval) {
        interval = setInterval(updateChart, 500);
      }
    }
  });

  onDestroy(() => {
    // console.log("On destroy called for grid sim");
    if (interval) {
      clearInterval(interval);
      interval = null;
      // console.log("Interval cleared for grid sim.");
    }
    if (chart) {
      chart.destroy();
      chart = null;
      // console.log("Chart destroy called for grid sim");
    }
  });

  $: if (data && data.oscilloscope_detail) {
    initialized = true;
    updateChart();
    // console.log("Reactive if was triggered... [chart2]");
    // console.log(
    //   "Data oscilloscope detail is below, should be amplitude frequency phase"
    // );
    // console.log(data.oscilloscope_detail);
  }

  function generateSineWave(
    amplitude,
    frequency,
    phase,
    duration = 4,
    sampleRate = 1000
  ) {
    const datasin = [];
    const labels = [];
    const angularfreq = 2 * Math.PI * frequency;
    const increment = 1 / sampleRate;

    for (let t = 0; t < duration / 100; t += increment) {
      const value = amplitude * Math.sin(angularfreq * t + phase);
      datasin.push({ x: t * 1000, y: value });
      labels.push((t * 1000).toFixed(0));
    }

    return { datasin, labels };
  }

  let iteration = 0;
  function updateChart() {
    if (!initialized || !chart) {
      return;
    }

    const currentTime = Date.now();
    const offset = currentTime % (duration * 1000);
    // currentTime += 1;
    iteration += 0.5;
    // console.log(iteration);

    const { amplitude, frequency, phase } = data.oscilloscope_detail;

    const jedandva = (2 * Math.PI) / 3;

    let phase1 = iteration;
    let phase2 = jedandva + iteration;
    let phase3 = (4 * Math.PI) / 3 + iteration;

    // console.log(phase1);
    // console.log(phase2);
    // console.log(phase3);

    const result1 = generateSineWave(
      amplitude,
      frequency,
      phase1,
      duration,
      sampleRate
    );
    const result2 = generateSineWave(
      amplitude,
      frequency,
      phase2,
      duration,
      sampleRate
    );
    const result3 = generateSineWave(
      amplitude,
      frequency,
      phase3,
      duration,
      sampleRate
    );

    datasin = result1.datasin;
    datasin2 = result2.datasin;
    datasin3 = result3.datasin;

    // console.log("This is result 1: ", result1);

    chart.data.labels = result1.labels;
    chart.data.datasets[0].data = datasin.map((point) => point.y);
    chart.data.datasets[1].data = datasin2.map((point) => point.y);
    chart.data.datasets[2].data = datasin3.map((point) => point.y);

    chart.update();
  }
</script>

<div class="flex min-w-full min-h-full">
  <canvas bind:this={chartCanvas} height="310"></canvas>
</div>
