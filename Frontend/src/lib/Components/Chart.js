import { Chart } from "chart.js/auto";

export function createChart(context, config) {
  return new Chart(context, config);
}

export const sampleChartConfig = {
  type: "line",
  data: {
    labels: [
      "0",
      "1",
      "2",
      "3",
      "4",
      "5",
      "6",
      "8",
      "9",
      "10",
      "11",
      "12",
      "13",
      "14",
      "15",
      "16",
      "17",
      "18",
      "19",
      "20",
    ],
    datasets: [
      {
        fill: true,
        label: "Price",
        data: [],
        backgroundColor: [
          "rgba(0, 0, 255, 0.1)",
          // "rgba(54, 162, 235, 0.2)",
          // "rgba(255, 206, 86, 0.2)",
          // "rgba(75, 192, 192, 0.2)",
          // "rgba(153, 102, 255, 0.2)",
          // "rgba(255, 159, 64, 0.2)",
        ],
        borderColor: [
          "rgba(0, 0, 255, 1)",
          // "rgba(54, 162, 235, 1)",
          // "rgba(255, 206, 86, 1)",
          // "rgba(75, 192, 192, 1)",
          // "rgba(153, 102, 255, 1)",
          // "rgba(255, 159, 64, 1)",
        ],
        borderWidth: 1,
        pointRadius: 0,
      },
    ],
  },
  options: {
    scales: {
      x: {
        ticks: {
          maxTicksLimit: 5
        }
      },
      y: {
        beginAtZero: false,
      },
    },
    options: {
      responsive: true,
    },
    plugins: {
      legend: {
        display: false,
      },
    },
  },
};
