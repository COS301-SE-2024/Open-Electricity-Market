<script>
  import { onMount } from "svelte";
  import Chart from "$lib/Components/Chart.svelte";
  import Cookies from 'js-cookie';
  

  let data = {};
  let interval; 
  let advancedView = false; 
  let numDecimals = 2; 
  $: nodes = [1,2,3,4,5,6];
  let amount; 
  let withdrawamount; 
  let totalamount = 0; 
  let firstname; 
  let lastname; 
  let email; 

  onMount(async () => {
    await fetchStart();
    await fetchData();
    await getUserDetails();
    interval = setInterval(fetchData, 10000);

    //return function runs when the component is unmounted 
    return() => {
      clearInterval(interval);
    };
  }); 

  async function fetchStart() {
    try {
      const response = await fetch("http://localhost:8000/start", {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json' 
      }
    });
  }
    catch(error){
      console.log("There was an error sending a post to /start endpoint.");
    }
  };
 async function fetchData() {

    try {
      const response = await fetch("http://localhost:8000/overview", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json', 
          'Accept': 'application/json',
        }
      });
      // console.log("request being sent...");
      // console.log(response);
      const fdata = await response.json();
      // console.log(data);

      //Voltage 1,2,3 as well as price
      data = fdata; 
    } catch (error) {
      // console.log("There was an error fetching the JSON for the overview..", error);
    }
  };

  function createModal(){
    document.getElementById("newNodeModal").showModal();
  }

  function createNode() {
    // if all fields filled in
    document.getElementById("newNodeModal").close();

    // submit the new node request and update the nodes dynamic nodes array
  }

  async function addFunds(){

    
    if(!amount){
      console.log("No amount was given.");
      return; 
    }

    console.log("Add funds function was called " + amount);
    try {
      const response = await fetch("http://localhost:8001/add_funds", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        },
        body: JSON.stringify({
            funds: amount
          }),
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      console.log("Data received from add funds endpoint is this: ", data);
      
    } catch (error) {
      console.log("There was an error fetching the JSON for the add funds endpoint:", error);
    }

    //if funds added then show confirmation modal
    if(data.message == 'Funds added'){
      document.getElementById("addfundsconfirmation").showModal();
      // amount = '';
      totalamount += amount; 
    }


  }


  async function withdrawFunds(){

    
    if(!withdrawamount){
      console.log("No amount was given.");
      return; 
    }

    try {
      const response = await fetch("http://localhost:8001/remove_funds", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        },
        body: JSON.stringify({
            funds: withdrawamount
          }),
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      console.log("Data received from withdraw funds endpoint is this: ", data);
      
    } catch (error) {
      console.log("There was an error fetching the JSON for the withdrawfunds:", error);
    }

    //if funds added then show confirmation modal
    if(data.message == 'Funds removed'){
      document.getElementById("removefundsconfirmation").showModal();
      // withdrawamount = '';
      totalamount -= withdrawamount; 
    }



  }



  async function getUserDetails(){

    
    try {
      const response = await fetch("http://localhost:8001/user_details", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        },
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      console.log("Data received from user details is: ", data);
      
    } catch (error) {
      console.log("There was an error fetching user details:", error);
    }

    if(data.message == "User details successfully retrieved"){
      totalamount = data.data.credit; 
      email = data.data.email; 
      firstname = data.data.first_name; 
      lastname = data.data.last_name;
    }



  }


  async function removeAccount(){

    try {
      const response = await fetch("http://localhost:8001/remove_account", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        },
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      console.log("Data received from remove account endpoint: ", data);
      
    } catch (error) {
      console.log("There was an error calling remove account:", error);
    }

    if(data.message = "Account successfully deleted"){
      Cookies.remove("session_id");
      window.location.href = '/login';
    }


  }

  function nullifyvalues(){
    withdrawamount = '';
    amount = '';
  }


  

</script>

<main class="container mx-auto w-full flex justify-center">

            <button class="btn btn-success" onclick="add_modal.showModal()">Add funds</button>
            <button class="btn btn-error" onclick="remove_modal.showModal()">Withdraw funds</button>
            <button class="btn btn-error" onclick="removeaccount_modal.showModal()">Remove Account</button>

            <dialog id = "add_modal" class="modal">
              <div class="modal-box">
                <h3 class="text-lg font-bold">Add funds</h3>
                <p class="py-4">Please enter an amount you would like to add.</p>
                    <div class="form-control mt-4">
                     <input class="input input-bordered" type="number" placeholder="Amount" required bind:value={amount}>
                    </div>
             
                <div class="modal-action">
                  <form method="dialog">
                    <button class="btn bg-green-600" on:click="{addFunds}">Continue</button>
                    <button class="btn bg-red-600">Cancel</button>
                  </form>
                </div>
              </div>
            </dialog>


            <dialog id="remove_modal" class="modal">
              <div class="modal-box">
                <h3 class="text-lg font-bold">Withdraw funds</h3>
                <p>Please ente ran amount you would like to withdraw.</p>
                 <div class="form-control mt-4">
                     <input class="input input-bordered" type="number" placeholder="Amount" required bind:value={withdrawamount}>
                    </div>
                <div class="modal-action">
                  <form method="dialog">
                    <button class="btn bg-green-600" on:click={withdrawFunds}>Continue</button>
                    <button class="btn bg-red-500">Cancel</button>
                  </form>
                </div>
              </div>
            </dialog>




  <div class="max-w-min mx-20">
    <div class="stats stats-vertical"> 
      <div class="stat">
        <div class="stat-title">Available Credit</div>
        <div class="stat-value">R{totalamount}</div>
      </div>
    
      <div class="stat">
        <div class="stat-title">Pending Transactions</div>
        <div class="stat-value">5</div>
      </div>
      
      <div class="stat">
        <div class="stat-title">Total Comsumption</div>
        <div class="stat-value">1,024W</div>
      </div>
    
      <div class="stat">
        <div class="stat-title">Total Generation</div>
        <div class="stat-value">5W</div>
      </div>
    </div>

    <div class="card">
      <div class="card-title">Personal Information</div>
    </div>

    <div class="stat">
        <div class="stat-title">Firstname</div>
        <div class="stat-value">{firstname}</div>
    </div>

    <div class="stat">
        <div class="stat-title">Lastname</div>
        <div class="stat-value">{lastname}</div>
    </div>

    <div class="stat">
        <div class="stat-title">Email</div>
        <div class="stat-value">{email}</div>
    </div>
    
  </div>

  <div class="min-w-max min-h-fit mx-10 flex-row">

    <div class="flex-col">
      <span class="text-3xl justify-start pl-2">
        Your Nodes
      </span>
      <span class="justify-end pl-96">
        <button class="btn btn-primary text-lg " on:click={createModal}>Add a Node</button>
      </span>
    </div>

    <dialog id="newNodeModal" class="modal">  
      <div class="modal-box">
        <h3 class="font-bold text-lg ">Add a Node</h3>
        <form class="">
          <div class="form-control mt-4">
            <input class="input input-bordered" type="text" placeholder="Name">
          </div>
          <div class="form-control mt-4">
            <input class="input input-bordered" type="text" placeholder="Latitude">
          </div>
          <div class="form-control mt-4">
            <input class="input input-bordered" type="text" placeholder="Longtitude">
          </div>
          <div class="form-control mt-4">
            <button class="btn btn-primary" on:click={createNode}>Confirm</button>
          </div>
        </form>
      </div>


      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog>
    

    {#each nodes as node}
      <div class="card card-side min-w-1/3 bg-base-300 my-2">
        <figure class="w-1/5 p-10">
          <img
            src="../src/images/house.png"
            alt="House node" />
        </figure>
        <div class="card-body">
          <h2 class="card-title">Node {node}</h2>
          <p class="">
            Node type, etc
          </p>
          <div class="card-actions justify-end">
            <button class="btn btn-ghost">Details</button>
          </div>
        </div>
      </div>

    {/each}


    
  </div>


  <dialog id="addfundsconfirmation" class="modal">  
      <div class="modal-box">
        <h3 class="font-bold text-lg ">You have successfully added funds!</h3>
      <p>You have successfully added R{amount} to your account.</p>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button on:click={nullifyvalues}>close</button>
      </form>
    </dialog>


    <dialog id="removefundsconfirmation" class="modal">  
      <div class="modal-box">
        <h3 class="font-bold text-lg ">Withdrawal of funds successful!</h3>
      <p>You have successfully withdrew R{withdrawamount} from your account.</p>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button on:click={nullifyvalues}>close</button>
      </form>
    </dialog>

      
      <dialog id="removeaccount_modal" class="modal">
      <div class="modal-box">
        <h3 class="text-lg font-bold">Delete Account</h3>
        <p class="py-4">Are you sure you would like to delete your account?</p>
        <div class="modal-action">
          <form method="dialog">
            <!-- if there is a button in form, it will close the modal -->
            <button class="btn bg-red-500" on:click={removeAccount}>Delete</button>
            <button class="btn bg-gray-600">Cancel</button>
          </form>
        </div>
      </div>
    </dialog>

</main>

