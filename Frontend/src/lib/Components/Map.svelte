<script>
  import { onMount, onDestroy, createEventDispatcher } from "svelte";
  import { browser } from "$app/environment";
  import Chart from "./Chart2.svelte";
  import { API_URL_GRID, API_URL_MARKET } from "$lib/config.js";
  import iconmarkerpng from "$lib/assets/marker-icon.png";

  let mapContainer;
  let map;
  let markerIcon;

  let data = {};
  let markers = [];
  export let mapdata;
  const dispatch = createEventDispatcher();

  onMount(async () => {
    if (browser) {
      const leaflet = await import("leaflet");
      map = leaflet.map(mapContainer).setView([-26.1925013, 28.0100383], 13);

      leaflet
        .tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
          attribution:
            '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
        })
        .addTo(map);

      markerIcon = leaflet.icon({
        iconUrl: iconmarkerpng,
        iconSize: [25, 41],
        iconAnchor: [12, 41],
        popupAnchor: [1, -34],
        shadowSize: [41, 41],
        shadowAnchor: [12, 41],
      });
    }

    updateMarkers(mapdata);

    return () => {
      if (map) {
        map.remove();
      }
    };
  });

  onDestroy(async () => {});

  function updateMarkers() {
    if (!mapdata.loads || !mapdata.generators) {
      console.log(mapdata);
      console.log("No loads or generators available");
      return;
    }

    markers.forEach((marker) => marker.remove());
    markers = [];

    mapdata.loads.forEach((load) => {
      if (load.load_type.Consumer) {
        const consumer = load.load_type.Consumer;
        const marker = L.marker(
          [consumer.location.longitude, consumer.location.latitude],
          { icon: markerIcon }
        ).addTo(map);

        marker.bindPopup(
          "Consumer " +
            (load.id +
              1 +
              "<br>" +
              consumer.location.longitude +
              " " +
              consumer.location.latitude)
        );
        // marker.on("click", () => showMarkerPopup(marker, consumer));
        //marker.on('click', ()=> updateChart(consumer));
        marker.on("click", () => {
          dispatch("markerClick", consumer);
        });
        markers.push(marker);
      }
    });

    // These markers are usually in the same positions as the load markers, and cover them completely
    // mapdata.generators.forEach((generator) => {
    //   const marker = L.marker([
    //     generator.location.longitude,
    //     generator.location.latitude,
    //   ]).addTo(map);
    //   marker.bindPopup(
    //     "Generator " +
    //       (generator.id +
    //         1 +
    //         "<br>" +
    //         generator.location.longitude +
    //         " " +
    //         generator.location.latitude)
    //   );
    //   // marker.on("click", () => showMarkerPopup(marker, generator));
    //   markers.push(marker);
    // });
  }

  async function showModal() {
    document.getElementById("test_modal").showModal();
  }

  function updateChart(entity) {
    if (entity.voltage.oscilliscope_detail) {
      console.log("This was successful");
    }
  }

  // function extractChartData(data){
  //     let chartData = [];
  //     if(data.Consumers){
  //         chartData = data.Consumers[0].Voltage["Phase 1"];
  //     }
  //     return chartData;
  // }

  // $: if(data){
  //     updateMarkers();
  // }

  // $: if (map && mapdata && browser) {
  //   console.log("Reactive if was triggered...");
  //   updateMarkers(mapdata);
  // }

  function resizeMap() {
    if (browser) {
      if (window.innerWidth <= 450) {
        // chart.style.width = '100%';
        mapContainer.style.height = "350px";
        mapContainer.style.width = "290px";
        // chartCanvas.style.width = '300px';
        // chartCanvas.style.width = '200px';
        console.log("If statement is running...");
      } else {
        mapContainer.style.height = "700px";
        // chartCanvas.style.width = '900px';
        console.log("Else was executed...");
        // chart.style.height = '600px';
      }
    }
  }
</script>

<main class="min-w-full min-h-full">
  <div bind:this={mapContainer}></div>
</main>

<style>
  @import "leaflet/dist/leaflet.css";
  div {
    height: 700px;
    z-index: 0;
  }
</style>
