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
      "21",
      "22",
      "23",
      "24",
    ],
    datasets: [
      {
        fill: true,
        label: "Predicted Price",
        data: [],
        backgroundColor: ["rgba(0, 255, 0, 0.3)"],
        borderColor: ["rgba(0, 255, 25, 1)"],
        borderWidth: 1,
        pointRadius: 0,
      },
    ],
  },
  options: {
    scales: {
      y: {
        beginAtZero: true,
      },
      x: {
        title: {
          display: true,
          text: "Time (Minutes)", 
        },
      },
      y: {
        beginAtZero: true,
        title: {
          display: true,
          text: "Price per kW (ZAR)", 
        },
      },
    },
    options: {
      responsive: true,
    },
    
  },
};