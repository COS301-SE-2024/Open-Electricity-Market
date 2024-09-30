<script>

    import { onMount, onDestroy } from "svelte";
    import { createChart, config } from './PieChart.js'; 

    let chart; 
    let chartCanvas; 
    export let agentpiedata; 
    let interval; 

    function resizeRadar(){
        if(window.innerWidth <= 760){
            chartCanvas.style.height = '320px'; 
        }
        else{
            // chartCanvas.style.height = '700px'; 
        }
    }

    onMount(async () => {

        if(typeof window !== 'undefined'){
            chart = createChart(chartCanvas, config); 
            // fillChart(); 
        }
        window.addEventListener('resize', resizeRadar); 

        return () => {
            if(chart){
                chart.onDestroy(); 
            }
            window.removeEventListener('resize', resizeRadar); 
        }

    }); 

    

     $: if (chart && agentpiedata && Object.keys(agentpiedata).length > 0) {
         console.log("Agent piechart data was successfully placed"); 
         chart.data.labels = ["Units Consumed", "Units Produced"];
         chart.data.datasets[0].data = [agentpiedata.unitsconsumed, agentpiedata.unitsproduced]; 
         updateChart();
         
  }


  async function fillChart(){
    console.log("data is this: ", data);
    console.log("unitsbought is this: ", data.unitsbought);  
    if (chart && data.length > 0) {
        console.log("this if was triggered"); 
        chart.data.labels = ["Units Bought", "Units Sold"];
        chart.data.datasets[0].data = [data.unitsbought, data.unitssold]; 
        updateChart();
    }
  }

  function updateChart(){
    chart.update(); 
  }



        




</script>



<div style="display: flex; max-height: 100%" >
    <canvas bind:this={chartCanvas} height = 340px ></canvas> 
</div>