
<script>
    import { onMount } from 'svelte';
    import { createChart, sampleChartConfig } from './Chart.js';
  
    let chart;
    let chartCanvas;
  
    onMount(() => {
      if (typeof window !== 'undefined') { // Check if running in the browser
        chart = createChart(chartCanvas, sampleChartConfig);
      }
      
      return () => {
        if (chart) {
          chart.destroy();
      
        }
      };
    });
  </script>
  
  <canvas bind:this={chartCanvas} width="400" height="400"></canvas>
  
  <style>
    canvas {
      max-width: 100%;
      max-height: 100%;
    }
  </style>
  