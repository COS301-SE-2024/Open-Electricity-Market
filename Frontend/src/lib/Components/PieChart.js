import { Chart } from "chart.js/auto";

export function createChart(context, config) {
  return new Chart(context, config);
}

const data = {
  labels: ["Red", "Blue", "Yellow"],
  datasets: [
    {
      label: "Market Statistics",
      data: [300, 50, 100],
      backgroundColor: [
        "rgb(255, 99, 132)",
        "rgb(54, 162, 235)",
        "rgb(255, 205, 86)",
      ],
      hoverOffset: 4,
    },
  ],
};

export const config = {
  type: "doughnut",
  data: data,
};
