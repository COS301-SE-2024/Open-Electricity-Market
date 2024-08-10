
<script>
     
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import { browser } from '$app/environment';
  import Chart from './Chart2.svelte';
  import Chart2 from './Chart2.svelte';
  // import leaflet from 'leaflet'; 
  import iconmarkerpng from '../../images/marker-icon.png';

  let mapContainer;
  let map;
  let lm; 
  
  let interval; 
  let data = {};
  let markers = [];

  const dispatch = createEventDispatcher();


  
  onMount(async () => {
    if(browser) {
      const leaflet = await import('leaflet');
      
      map = leaflet.map(mapContainer, {
        scrollWheelZoom: false
      }).setView([-26.2225013,28.0523383], 12);

      
      leaflet.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', { attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(map);

      const markerIcon = leaflet.icon({
        iconUrl: iconmarkerpng,
        iconSize: [25, 41], 
        iconAnchor: [12, 41], 
        popupAnchor: [1, -34], 
        shadowSize: [41, 41], 
        shadowAnchor: [12, 41]
      });
      
      lm = leaflet.marker([-26.1925013,28.0100383], {icon:markerIcon}).addTo(map);
      leaflet.marker([-26.1825013, 28.0023383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.1725013, 28.0223383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.2025013, 28.0323383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.2125013, 28.0123383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.1925013, 28.0423383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.2225013, 28.0523383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.2325013, 28.0223383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.2025013, 28.0623383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.2425013, 28.0723383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.2525013, 28.0323383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.2625013, 28.0423383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.2725013, 28.0123383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.1925013, 28.0823383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.2825013, 28.0223383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.1925013, 28.0923383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.2925013, 28.0123383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.1825013, 28.1023383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.1725013, 28.1123383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.2025013, 28.1223383], {icon: markerIcon}).addTo(map);
      leaflet.marker([-26.2125013, 28.1323383], {icon: markerIcon}).addTo(map);

    
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
    height: 400px;
    padding: 8;
  }
</style>


<dialog id="test_modal" class="modal">  
  <div class="modal-box">
    <h3 class="font-bold text-lg ">Voltage</h3>
    <Chart {data}/>
  </div>
  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>

