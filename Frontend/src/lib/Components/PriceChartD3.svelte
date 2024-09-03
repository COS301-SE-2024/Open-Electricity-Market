<script>
  import { onMount } from 'svelte';
  import { createChart } from 'lightweight-charts';

  onMount(() => {
    const chartElement = document.getElementById('chartPrice');
    if (chartElement) {
      const chart = createChart(chartElement, {
        width: 600,
        height: 400,
        layout: { textColor: 'black', background: { type: 'solid', color: 'white' } }
      });

      const lineSeries = chart.addLineSeries({ color: '#2962FF' });
      lineSeries.setData([
        { time: '2024-01-01', value: 32.51 },
        { time: '2024-01-02', value: 31.11 },
        
      ]);

      lineSeries.update({time: '2024-01-03', value: 30.12}); 
      lineSeries.update({time: '2024-01-04', value: 39.12}); 
      lineSeries.update({time: '2024-01-05', value: 36.12}); 
      lineSeries.update({time: '2024-01-06', value: 23.12}); 
      lineSeries.update({time: '2024-01-07', value: 43.12}); 
      lineSeries.update({time: '2024-01-08', value: 23.12}); 

      chart.timeScale().fitContent();
    } else {
      console.error('Element with ID "chartPrice" not found.');
    }
  });
</script>

<div id="chartPrice" style="width: 600px; height: 400px;"></div>
