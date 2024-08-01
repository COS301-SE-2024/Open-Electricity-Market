
<script>
     
    import { onMount, onDestroy, createEventDispatcher } from 'svelte';
    import { browser } from '$app/environment';
    import Chart from './Chart2.svelte';
    import {tick} from 'svelte';
    
    let mapContainer;
    let map;
    let lm; 
    
    let interval; 
    let data = {};
    let markers = [];
    export let mapdata; 
    const dispatch = createEventDispatcher();


    
    onMount(async () => {
       if(browser) {
          const leaflet = await import('leaflet');
          map = leaflet.map(mapContainer, {
            scrollWheelZoom: false
          }).setView([-26.1925013,28.0100383], 13);
    

    leaflet.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', { attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(map);
    
       }
    
    });

   
    
    onDestroy(async () => {
       if(map) {
          map.remove();
       }
    });

     async function fetchData() {
    try {
      const response = await fetch("http://localhost:8000/info", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        }
      });
    
      
      
    } catch (error) {
      console.log("There was an error fetching the JSON for the info:", error);
    }
  }

    

    function updateMarkers(){

          if (!data.loads || !data.generators) {
            console.log("No loads or generators available");
            return;
        }

        markers.forEach(marker=>marker.remove());
        markers = [];

        

        data.loads.forEach(load => {
        if (load.load_type.Consumer) {
          const consumer = load.load_type.Consumer;
          const marker = L.marker([consumer.location.latitude, consumer.location.longitude]).addTo(map);
          
          marker.bindPopup("Consumer "+ (consumer.id+1+"<br>"+consumer.location.latitude + " " + consumer.location.longitude));
         
          marker.on('click', () => {dispatch('markerClick', consumer)});
          markers.push(marker);
          }
        });

        data.generators.forEach(generator => {
          const marker = L.marker([generator.location.latitude, generator.location.longitude]).addTo(map);
          // marker.on('click', () => showMarkerPopup(marker, generator));
          markers.push(marker);
        });

    }



   async function showModal(){
      
        document.getElementById("test_modal").showModal();
      
    }

    function updateChart(entity){
      if(entity.voltage.oscilliscope_detail){
        console.log("This was successful");
      }
    }

  

     $: if (map && mapdata) {
    console.log("Reactive if was triggered...");
    updateMarkers(mapdata);
  }


  

   


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



    <dialog id="test_modal" class="modal">  
        <div class="modal-box">
          <h3 class="font-bold text-lg ">Voltage</h3>
          <Chart {data}/>
        </div>
        <form method="dialog" class="modal-backdrop">
          <button>close</button>
        </form>
      </dialog>

