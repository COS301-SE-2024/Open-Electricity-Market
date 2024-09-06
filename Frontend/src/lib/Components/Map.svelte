<script>
  import { onMount, onDestroy, createEventDispatcher } from "svelte";
  import { browser } from "$app/environment";
  import iconmarkerpng from "$lib/assets/marker-icon.png";

  let mapContainer;
  let map;
  let markerIcon;

  let markers = [];
  export let mapdata;
  const dispatch = createEventDispatcher();

  onMount(async () => {
    if (browser) {
      const leaflet = await import("leaflet");
      map = leaflet.map(mapContainer).setView([-25.7975, 28.2285], 11);

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
      if (load.id == 0) return;

      if (load.load_type.Consumer) {
        const consumer = load.load_type.Consumer;
        const marker = L.marker(
          [consumer.location.longitude, consumer.location.latitude],
          { icon: markerIcon }
        ).addTo(map);

        marker.bindPopup(
          "Consumer " +
            (load.id +
              "<br>" +
              consumer.location.longitude +
              " " +
              consumer.location.latitude)
        );

        let generators = [];
        mapdata.generators.forEach((generator) => {
          if (
            consumer.location.longitude == generator.location.longitude &&
            consumer.location.latitude == generator.location.latitude
          ) {
            generators.push(generator);
          }
        });

        // marker.on("click", () => showMarkerPopup(marker, consumer));
        // marker.on('click', ()=> updateChart(consumer));
        marker.on("click", () => {
          consumer["generators"] = generators;
          dispatch("markerClick", consumer);
        });
        markers.push(marker);
      }
    });

    // These markers are usually in the same positions as the load markers, and cover them completely
    // Might need to add any generators that do not have corresponding loads
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
    //   marker.on("click", () => {
    //     generator["type"] = "generator";
    //     dispatch("markerClick", generator);
    //   });
    //   markers.push(marker);
    // });
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
