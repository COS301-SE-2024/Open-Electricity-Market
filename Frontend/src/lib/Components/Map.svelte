
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
          map = leaflet.map(mapContainer).setView([-26.1925013,28.0100383], 13);
    

    leaflet.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', { attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(map);
    
    
    lm = leaflet.marker([-26.1925013,28.0100383]).addTo(map);
    
    lm.on('click', () => showModal());
    





    
       }
       await fetchData();
       interval = setInterval(fetchData, 10000);
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
      //console.log("Request being sent...");
      const fdata = await response.json();
      //console.log("Fetched data:", fdata);
      data = fdata.circuits[0] || {};
      //console.log("This is circuits...");
      //console.log(data);
      updateMarkers();
      
      
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

        //consumers
        // data.Consumers.forEach(consumer=>{
        //     const marker = lm.marker([consumer.lattitude, consumer.longtitude]).addTo(map);
        //     marker.on('click', ()=> showMarkerPopup(marker, consumer));
        //     markers.push(marker);
        // });

        // //transformers 
        // data.Transformers.forEach(transformer=>{
        //     const marker = lm.marker([transformer.lattitude, transformer.longtitude]).addTo(map);
        //     marker.on('click', ()=> showMarkerPopup(marker, transformer));
        //     markers.push(marker);
        // });

        // //generators
        // data.Generators.forEach(generator=>{
        //     const marker = lm.marker([generator.lattitude, generator.longtitude]).addTo(map);
        //     marker.on('click', ()=> showMarkerPopup(marker, generator));
        //     markers.push(marker);
        // });

        // //generators
        // data["Transmission Lines"].forEach(line=>{
        //     const marker = lm.marker([line.lattitude, line.longtitude]).addTo(map);
        //     marker.on('click', ()=> showMarkerPopup(marker, line));
        //     markers.push(marker);
        // });

        data.loads.forEach(load => {
        if (load.load_type.Consumer) {
          const consumer = load.load_type.Consumer;
          const marker = L.marker([consumer.location.latitude, consumer.location.longitude]).addTo(map);
          
          marker.bindPopup("Consumer "+ (consumer.id+1+"<br>"+consumer.location.latitude + " " + consumer.location.longitude));
          // marker.on('click', () => showMarkerPopup(marker, consumer));
          //marker.on('click', ()=> updateChart(consumer));
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

