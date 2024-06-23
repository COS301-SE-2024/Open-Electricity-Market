<script>
    import logo from '$lib/assets/Logo.png';
    import {goto} from "$app/navigation";
    let email = '';
    let firstname = '';
    let surname = '';
    let password = '';
    let passwordValidate = '';
    let errormess = "";
    async function create(){
        //add to database
        if(password == passwordValidate)
        {
          const res = await fetch("http://localhost:8001/register", {
            method: "POST", 
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
              "email": email,
              "first_name": firstname,
              "last_name": surname, 
              "password": password
            })
          });
          const json = await res.json();
          if(json.status == "ok")
          {
            goto("/");
          }
          else
          {
            errormess = "An error occured";
          }
        } else {
          errormess = "Passwords must match"
        }
    }
    
    function back(){
        goto("/");
    }
</script>
<main>
    <div class="hero min-h-screen" style="background-image: url(https://images.unsplash.com/photo-1510595256055-e44b27efe497?q=80&w=1700&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D);">
        <div class="hero-overlay bg-opacity-60 "></div>
        <div class="hero-content flex-col lg:flex-row-reverse rounded-2xl bg-base-100">
          <div class="text-center lg:text-left">
            <h1 class="text-5xl font-bold">Open Electricity Market.</h1>
            <p class="py-6">Discover a revolutionary platform where you can buy and sell electricity in real-time.</p>
          </div>
          <div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
            <form class="card-body">
              
              <h2 class="text-base font-semibold"> Create an account </h2>

              <div class="form-control">
                <input type="email" placeholder="Email" class="input input-bordered" required bind:value={email}/>
              </div>

              <div class="form-control mt-4">
                <input type="text" placeholder="First name" class="input input-bordered" required bind:value={firstname}/>
              </div>

              <div class="form-control mt-4">
                <input type="text" placeholder="Surname" class="input input-bordered" required bind:value={surname}/>
              </div>
              
              <div class="form-control mt-4">
                <input type="password" placeholder="Password" class="input input-bordered" required bind:value={password}/>
              </div>

              <div class="form-control mt-4">
                <input type="password" placeholder="Re-enter password" class="input input-bordered" required bind:value={passwordValidate}/>
              </div>

              {#if errormess != ''}
                <h2 class="text-base font-semibold text-black bg-error rounded"> { errormess } </h2>
              {/if}

              <div class="form-control mt-4">
                <button class="btn btn-primary" on:click={create}>Create account</button>
              </div>
              
              <div class="form-control mt-3">
                <button class="btn btn-primary" on:click={back}>I already have an account</button>
              </div>
            </form>
          </div>
        </div>
      </div>
</main>