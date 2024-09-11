<script>
  import { onMount, onDestroy } from "svelte";
  import { browser } from "$app/environment";
  import iconmarkerpng from "$lib/assets/marker-icon.png";

  let mapContainer;
  let map;
  let marker;
  export let onMapClick;

  onMount(async () => {
    if (browser) {
      const leaflet = await import("leaflet");
      map = leaflet
        .map(mapContainer, {
          scrollWheelZoom: false,
        })
        .setView([-25.7975, 28.2285], 11);

      leaflet
        .tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
          attribution:
            '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
        })
        .addTo(map);

      const markerIcon = leaflet.icon({
        iconUrl: iconmarkerpng,
        iconSize: [25, 41],
        iconAnchor: [12, 41],
        popupAnchor: [1, -34],
        shadowSize: [41, 41],
        shadowAnchor: [12, 41],
      });

      map.on("click", function (e) {
        const { lat, lng } = e.latlng;
        if (marker) {
          map.removeLayer(marker);
        }
        marker = L.marker([lat, lng], { icon: markerIcon }).addTo(map);
        if (onMapClick) {
          onMapClick(lng, lat);
        }
      });
    }
  });

  onDestroy(async () => {
    if (map) {
      map.remove();
    }
  });
</script>

<main>
  <div bind:this={mapContainer}></div>
</main>

<style>
  @import "leaflet/dist/leaflet.css";
  div {
    height: 500px;
  }
</style>
