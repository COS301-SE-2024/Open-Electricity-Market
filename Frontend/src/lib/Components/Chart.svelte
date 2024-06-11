
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
  
  <div style="display: flex; max-width: 50%;"> 
    <canvas bind:this={chartCanvas} height="300"></canvas>
  </div>
  
  <style>
    canvas {
      max-width: 100%;
      max-height: 100%;
    }
  </style>
  