
<script>
  import { onMount, onDestroy } from 'svelte';
  import { createChart, sampleChartConfig } from './Chart2.js';
  

  let chart;
  let chartCanvas;
  export let data = {}; 
  let interval; 

  onMount(() => {
    
    if (typeof window !== 'undefined') { // Check if running in the browser
      chart = createChart(chartCanvas, sampleChartConfig);
      // fetchData(); 
      //updateChart(data);  
      // interval = setInterval(updateChart, 1000);
    }

    return () => {
      if (chart) {
        chart.destroy();
      }
      // clearInterval(interval);
    };
  });

  

  // async function fetchData() {

  //     try {
  //       const response = await fetch("http://localhost:8000/overview", {
  //     method: "POST", 
  //     headers: {
  //       'Content-Type': 'application/json' 
  //     }

  //   });
  //       console.log("request being sent...");
  //       // const response = fetch("http://localhost:8000");
  //       const data = await response.json();
  //       console.log(data);
  //       //Voltage 1,2,3 as well as price
  //       updateChart(data.Phase1, data.Phase2);
  //     } catch (error) {
  //       console.log("There was an error fetching the JSON for the chart..", error);
  //     }
  // };

    $: if (data && data.oscilloscope_detail) {
    console.log("Reactive if was triggered...");
    console.log("Data oscilloscope detail is below, should be amplitude frequency phase");
    console.log(data.oscilloscope_detail);
    //chart.data.labels = labels;
    //chart.data.datasets[0].data = [];
 
    const { amplitude, frequency, phase } = data.oscilloscope_detail;
    const { datasin, labels } = generateSineWave(amplitude, frequency, phase);
    const jedandva = ((2*Math.PI)/3);

    const result2 = generateSineWave(amplitude, frequency, jedandva);
    const result3 = generateSineWave(amplitude, frequency, ((4*Math.PI)/3));  
    
    const { datasin: datasin2, labels: labels2 } = result2;
    const { datasin: datasin3, labels: labels3 } = result3;

    if (!chart.data.datasets || chart.data.datasets.length < 3) {
      chart.data.datasets = [
      { label: 'Phase 1', borderColor: 'red', data: [] },
      { label: 'Phase 2', borderColor: 'green', data: [] },
      { label: 'Phase 3', borderColor: 'blue', data: [] },
      ];
    }
    //console.log("This is the sin data");
    //console.log(sineData);

    chart.data.labels = labels; 
    chart.data.datasets[0].data = datasin.map(point => point.y);
    chart.data.datasets[1].data = datasin2.map(point2 => point2.y);
    chart.data.datasets[2].data = datasin3.map(point3 => point3.y);
    chart.data.datasets[1].data.push(1);
    console.log(datasin2);
    console.log(datasin3);
    console.log(datasin);
    console.log(phase);
    chart.update();
  }

  




  // async function updateChart(data){

  //   //this will have to check for price once endpoint changes *************
  //   if(chart && data.voltage){     
  //     console.log("Chart can see voltage object...");
  //     //const consumer = data.Consumers[0];
  //     //chart.data.datasets[0].data.push(consumer.Voltage["Phase 1"]);
  //   //   console.log(consumer.Voltage["Phase 1"]);
  //     // chart.data.datasets[1].data.push(consumer.Voltage["Phase 2"]);
  //     // chart.data.datasets[2].data.push(consumer.Voltage["Phase 3"]);
  //     // chart.data.labels.push(chart.data.labels.length + 1); 
  //     const osd = data.voltage.oscilloscope_detail; 
  //     const sinwavedata = generateSineWaveData(osd.amplitude, osd.frequency, osd.phase);
  //     console.log("This is the sin wave data: ");
  //     console.log(sinwavedata.data);
  //     chart.data.datasets[0].data = []; 
  //     //chart.update(); 
  //     for(let i = 0; i<10; i++){
  //       chart.data.datasets[0].data.push(1);
  //       chart.data.labels.push(chart.data.labels.length + 1);
  //     }


  //     //chart.data.labels.push(sinwavedata.labels);
  //     chart.update();
  //   }
  //   else{
  //       console.log("no update occurring");
  //   }
  //     return; 

  // }



   function generateSineWave(amplitude, frequency, phase, duration = 4, sampleRate = 1000) {

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


  let currentTime = 0; 
  let duration = 4; 

  function updateChart() {
    if(chart && chart.data.datasets[0].data!=[]){

      currentTime += 1; 
      const newData = generateSineWave(400, 0.5, 0, duration, 1000, currentTime);
      chart.data.labels = newData.labels;
      chart.data.datasets[0].data = newData.datasin.map(point => point.y);
      chart.options.scales.x.min = currentTime * 1000;
      chart.options.scales.x.max = (currentTime + duration) * 1000;
      chart.update();

    }
      
    }



 

</script>

<div style="display: flex; max-width: 100%;"> 
  <canvas bind:this={chartCanvas} height=280></canvas>
</div>

<style>
  /* canvas {
    max-width: 100%;
    max-height: 100%;
  } */
</style>
  