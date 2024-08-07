
<script>
     
    import { onMount, onDestroy, createEventDispatcher } from 'svelte';
    import { browser } from '$app/environment';
    import {tick} from 'svelte';
    
    let mapContainer;
    let map;
    let lm; 
    
    let interval; 
    let data = {};
    let marker;
    export let onMapClick;
    const dispatch = createEventDispatcher();


    
    onMount(async () => {
       if(browser) {
          const leaflet = await import('leaflet');
          map = leaflet.map(mapContainer, {
            scrollWheelZoom: false
          }).setView([-26.1925013,28.0100383], 13);
    

    leaflet.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', { attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(map);
    

        map.on('click', function (e) {
        const { lat, lng } = e.latlng;
        if (marker) {
        map.removeLayer(marker);
        }
        marker = L.marker([lat, lng]).addTo(map);
         if (onMapClick) {
        onMapClick(lng, lat);
      }
        });
       }
        
    
    });

   
    
    onDestroy(async () => {
       if(map) {
          map.remove();
       }
    });



    </script>
    
    
    <main>
       <div bind:this={mapContainer}></div>
    </main>
    
    <style>
       @import 'leaflet/dist/leaflet.css';
       div {
       height: 500px;
       }
    </style>



    

