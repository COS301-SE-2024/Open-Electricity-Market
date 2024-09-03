<script>

    import { onMount, onDestroy } from "svelte";
    import { createChart, config } from './PieChart.js'; 

    let chart; 
    let chartCanvas; 
    export let data = []; 
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
            
        }
        window.addEventListener('resize', resizeRadar); 

        return () => {
            if(chart){
                chart.onDestroy(); 
            }
            window.removeEventListener('resize', resizeRadar); 
        }

    }); 

    

     $: if (chart && data.length>0) {
     updateChart();
  }




        




</script>



<div style="display: flex; max-height: 100%" >
    <canvas bind:this={chartCanvas} height = 340px ></canvas> 
</div>