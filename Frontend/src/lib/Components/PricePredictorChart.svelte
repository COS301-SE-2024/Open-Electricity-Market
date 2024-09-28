<script>
    import { onMount, onDestroy } from "svelte";
    import { createChart, sampleChartConfig } from "./PricePredictorChart.js";
  
    let chart;
    let chartCanvas;
    export let data = [];
    let interval;
  
    function resizeChart() {
      if (window.innerWidth <= 760) {
        chartCanvas.style.height = "150px";
        chartCanvas.style.width = "150px";
       
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
        const interval = 180; 
        const currTime = new Date(); 
        const labels = data.map((_, index) => {
          const time = new Date(currTime.getTime() + index * 60000); 
          const hours = time.getHours().toString().padStart(2, '0');
          const minutes = time.getMinutes().toString().padStart(2, '0');
          return `${hours}:${minutes}`;
        });
        chart.data.datasets[0].data = data;
        chart.data.labels = labels; 
        chart.update();
      }
      return;
    }
  </script>
  
  <div class="chartstyle">
    <canvas bind:this={chartCanvas} height="450px" width = "650px"></canvas>
  </div>
  
  <style>
  
    .chartstyle{
      display: flex; 
      max-height: 100%; 
      
      /* @media (max-width:490px){
        margin-left: 0rem;
      } */
    }
  </style>
  