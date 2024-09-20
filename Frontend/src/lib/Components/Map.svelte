<script>
  import { onMount, onDestroy, createEventDispatcher } from "svelte";
  import { browser } from "$app/environment";
  import iconmarkerpng from "$lib/assets/marker-icon.png";

  let mapContainer;
  let map;
  let markerIcon;
  let transformerIcon;

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

      transformerIcon = leaflet.icon({
        iconUrl: iconmarkerpng,
        iconSize: [25, 41],
        iconAnchor: [12, 41],
        popupAnchor: [1, -34],
        shadowSize: [41, 41],
        shadowAnchor: [12, 41],
      });
    }

    updateMarkers();

    return () => {
      if (map) {
        map.remove();
      }
    };
  });

  onDestroy(async () => {});

  function updateMarkers() {
    mapdata.forEach((circuit) => {
      markers.forEach((marker) => marker.remove());
      markers = [];

      circuit.loads.forEach((load) => {
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
          circuit.generators.forEach((generator) => {
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
            consumer["type"] = "consumer";
            dispatch("markerClick", consumer);
          });
          markers.push(marker);
        }
      });

      circuit.transformers.forEach((transformer) => {
        if (transformer.id == 0) return;

        const marker = L.marker(
          [
            transformer.location.longtitude
              ? transformer.location.longtitude
              : 0,
            transformer.location.latitude ? transformer.location.latitude : 0,
          ],
          { icon: markerIcon }
        ).addTo(map);

        marker.bindPopup(
          "Transformer " +
            transformer.id +
            "<br>" +
            transformer.location.longitude +
            " " +
            transformer.location.latitude
        );

        marker.on("click", () => {
          transformer["type"] = "transformer";
          dispatch("markerClick", transformer);
        });
        markers.push(marker);
      });
    });

    // for each circuit (which is later connected to others via transformers)
    mapdata.forEach((circuit) => {
      // which has an array of connection internally
      circuit.connections.forEach((connection) => {
        // this is for storing the line starting and ending points
        var latlngs = Array();

        if (connection.Parallel) {
          // --- if the load ids are accessed under the "Parallel" key
          // find the load matching the first id
          var load = circuit.loads.find((l) => l.id === connection.Parallel[0]);

          // add the coordinates to the array:
          if (load.load_type.Consumer) {
            // if the load is a consumer
            latlngs.push([
              load.load_type.Consumer.location.longitude,
              load.load_type.Consumer.location.latitude,
            ]);
          } else {
            // if the load is a transmission line
            latlngs.push([
              load.load_type.TransmissionLine.location.longitude,
              load.load_type.TransmissionLine.location.latitude,
            ]);
          }

          // find the load matching the second id
          var load = circuit.loads.find((l) => l.id === connection.Parallel[1]);

          // add the coordinates to the array:
          if (load.load_type.Consumer) {
            // if the load is a consumer
            latlngs.push([
              load.load_type.Consumer.location.longitude,
              load.load_type.Consumer.location.latitude,
            ]);
          } else {
            // if the load is a transmission line
            latlngs.push([
              load.load_type.TransmissionLine.location.longitude,
              load.load_type.TransmissionLine.location.latitude,
            ]);
          }
        } else {
          // --- if the load ids are accessed under the "Series" key
          // find the load matching the first id
          var load = circuit.loads.find((l) => l.id === connection.Series[0]);

          // add the coordinates to the array:
          if (load.load_type.Consumer) {
            // if the load is a consumer
            latlngs.push([
              load.load_type.Consumer.location.longitude,
              load.load_type.Consumer.location.latitude,
            ]);
          } else {
            // if the load is a transmission line
            latlngs.push([
              load.load_type.TransmissionLine.location.longitude,
              load.load_type.TransmissionLine.location.latitude,
            ]);
          }

          // find the load matching the second id
          var load = circuit.loads.find((l) => l.id === connection.Series[1]);

          // add the coordinates to the array:
          if (load.load_type.Consumer) {
            // if the load is a consumer
            latlngs.push([
              load.load_type.Consumer.location.longitude,
              load.load_type.Consumer.location.latitude,
            ]);
          } else {
            // if the load is a transmission line
            latlngs.push([
              load.load_type.TransmissionLine.location.longitude,
              load.load_type.TransmissionLine.location.latitude,
            ]);
          }
        }
        // add the line to the map:
        var line = L.polyline(latlngs, { color: "black" }).addTo(map);
      });
    });
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
