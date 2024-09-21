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
    markers.forEach((marker) => marker.remove());
    markers = [];
    mapdata.forEach((circuit) => {
      circuit.loads.forEach((load) => {
        // if (load.id == 0) return;

        if (load.load_type.Consumer) {
          const consumer = load.load_type.Consumer;
          // console.log(consumer.location.latitude);
          const marker = L.marker(
            [consumer.location.latitude, consumer.location.longitude],
            { icon: markerIcon }
          ).addTo(map);

          marker.bindPopup(
            "Consumer" +
              "<br>" +
              consumer.location.latitude +
              " " +
              consumer.location.longitude
          );

          let generators = [];
          circuit.generators.forEach((generator) => {
            if (
              consumer.location.latitude == generator.location.latitude &&
              consumer.location.longitude == generator.location.longitude
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
        // if (transformer.id == 0) return;

        const marker = L.marker(
          [
            transformer.location.latitude,
            transformer.location.longtitude
              ? transformer.location.longtitude
              : 28.187,
          ],
          { icon: markerIcon }
        ).addTo(map);

        marker.bindPopup(
          "Transformer" +
            "<br>" +
            transformer.location.latitude +
            " " +
            transformer.location.longitude
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
              load.load_type.Consumer.location.latitude,
              load.load_type.Consumer.location.longitude,
            ]);
          } else {
            // if the load is a transmission line
            latlngs.push([
              load.load_type.TransmissionLine.location.latitude,
              load.load_type.TransmissionLine.location.longitude,
            ]);
          }

          // find the load matching the second id
          var load = circuit.loads.find((l) => l.id === connection.Parallel[1]);

          // add the coordinates to the array:
          if (load.load_type.Consumer) {
            // if the load is a consumer
            latlngs.push([
              load.load_type.Consumer.location.latitude,
              load.load_type.Consumer.location.longitude,
            ]);
          } else {
            // if the load is a transmission line
            latlngs.push([
              load.load_type.TransmissionLine.location.latitude,
              load.load_type.TransmissionLine.location.longitude,
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
              load.load_type.Consumer.location.latitude,
              load.load_type.Consumer.location.longitude,
            ]);
          } else {
            // if the load is a transmission line
            latlngs.push([
              load.load_type.TransmissionLine.location.latitude,
              load.load_type.TransmissionLine.location.longitude,
            ]);
          }

          // find the load matching the second id
          var load = circuit.loads.find((l) => l.id === connection.Series[1]);

          // add the coordinates to the array:
          if (load.load_type.Consumer) {
            // if the load is a consumer
            latlngs.push([
              load.load_type.Consumer.location.latitude,
              load.load_type.Consumer.location.longitude,
            ]);
          } else {
            // if the load is a transmission line
            latlngs.push([
              load.load_type.TransmissionLine.location.latitude,
              load.load_type.TransmissionLine.location.longitude,
            ]);
          }
        }
        // console.log("Connection: " + latlngs);
        // add the line to the map:
        L.polyline(latlngs, {
          color: "black",
          weight: 2,
          dashArray: "4 1 2",
        }).addTo(map);

        // ------------------------------------------

        // for each transformer within this circuit:
        circuit.transformers.forEach((transformer) => {
          // a line will be drawn in the order we put the locations into this array
          var latlngs = Array();

          // store the location of the load on the primary circuit
          var primary_load = mapdata.circuit[
            transformer.primary_circuit
          ].loads.find((l) => l.id === transformer.primary_load);
          // unfortunately it could be of any type of load:
          if (primary_load.load_type.Consumer) {
            latlngs.push([
              primary_load.load_type.Consumer.location.latitude,
              primary_load.load_type.Consumer.location.longitude,
            ]);
          } else {
            latlngs.push([
              primary_load.load_type.TransmissionLine.location.latitude,
              primary_load.load_type.TransmissionLine.location.longitude,
            ]);
          }

          // store the location of the transformer
          latlngs.push([
            transformer.location.latitude,
            transformer.location.longitude,
          ]);

          // store the location of the load on the secondary circuit
          // (always assumed to connect to the load with id = 0)
          var secondary_load = mapdata.circuit[
            transformer.secondary_circuit
          ].loads.find((l) => l.id === 0);
          if (secondary_load) {
            // there seems to be a chance that there is no second circuit
            // again
            if (secondary_load.load_type.Consumer) {
              latlngs.push([
                secondary_load.load_type.Consumer.location.latitude,
                secondary_load.load_type.Consumer.location.longitude,
              ]);
            } else {
              latlngs.push([
                secondary_load.load_type.TransmissionLine.location.latitude,
                secondary_load.load_type.TransmissionLine.location.longitude,
              ]);
            }
          }

          // add the line to the map:
          L.polyline(latlngs, { color: "black", weight: 2 }).addTo(map);
        });
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
