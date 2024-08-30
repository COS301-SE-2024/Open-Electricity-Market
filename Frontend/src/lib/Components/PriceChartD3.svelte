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
      const candleStickSeries = chart.addCandlestickSeries({
    upColor: '#26a69a', downColor: '#ef5350', borderVisible: false,
    wickUpColor: '#26a69a', wickDownColor: '#ef5350',
    });
      candleStickSeries.setData([
        { time: '2024-01-01', value: 32.51 },
        { time: '2024-01-02', value: 31.11 },
        
      ]);
      // candleStickSeries.update({time: '2024-01-03', value: 30.12}); 
      // candleStickSeries.update({time: '2024-01-04', value: 39.12}); 
      // candleStickSeries.update({time: '2024-01-05', value: 36.12}); 
      // candleStickSeries.update({time: '2024-01-06', value: 23.12}); 
      // candleStickSeries.update({time: '2024-01-07', value: 43.12}); 
      // candleStickSeries.update({time: '2024-01-08', value: 23.12}); 
      candleStickSeries.setData([
    { time: '2018-12-22', open: 75.16, high: 82.84, low: 36.16, close: 45.72 },
    { time: '2018-12-23', open: 45.12, high: 53.90, low: 45.12, close: 48.09 },
    { time: '2018-12-24', open: 60.71, high: 60.71, low: 53.39, close: 59.29 },
    { time: '2018-12-25', open: 68.26, high: 68.26, low: 59.04, close: 60.50 },
    { time: '2018-12-26', open: 67.71, high: 105.85, low: 66.67, close: 91.04 },
    { time: '2018-12-27', open: 91.04, high: 121.40, low: 82.70, close: 111.40 },
    { time: '2018-12-28', open: 111.51, high: 142.83, low: 103.34, close: 131.25 },
    { time: '2018-12-29', open: 131.33, high: 151.17, low: 77.68, close: 96.43 },
    { time: '2018-12-30', open: 106.33, high: 110.20, low: 90.39, close: 98.10 },
    { time: '2018-12-31', open: 109.87, high: 114.69, low: 85.66, close: 111.26 },
]);
      chart.timeScale().fitContent();
    } else {
      console.error('Element with ID "chartPrice" not found.');
    }
  });
</script>

<div id="chartPrice" style="width: 600px; height: 400px;"></div>