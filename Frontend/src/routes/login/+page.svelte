<script>
    import logo from '$lib/assets/Logo.png';
    import {goto} from "$app/navigation";
    let email = '';
    let password = '';
    let errormessage = "";

    // RFC 2822 standard email validation pattern
    var emailregex = /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/;


    function signup(){
      email = '';
      password = '';

      goto("/signup");
    }

    async function login(){

      if (!email.match(emailregex)) {
        errormessage = "Please enter a valid email address";
        return;
      }

      errormessage = "";

      const res = await fetch("http://localhost:8001/login", {
        method: "POST", 
        headers: {
          "Content-Type": "application/json",
          "Accept": "application/json",
        },
        body: JSON.stringify({
          "email": email, 
          "password": password
        })
      })
      const json = await res.json()
      //result = JSON.stringify(json)
      if(json.verified == true)
      {
        goto("/Main/Dashboard");
      }
      else{
        errormess = "Invalid Credentials";
      }
      //send to main page
    }
</script>

<main>
    <div class="hero min-h-screen" style="background-image: url(https://images.unsplash.com/photo-1510595256055-e44b27efe497?q=80&w=1700&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D);">
        <div class="hero-overlay bg-opacity-60 "></div>
        <div class="hero-content flex-col lg:flex-row-reverse rounded-2xl bg-base-100">
          <div class="text-center lg:text-left">
            <h1 class="text-5xl font-bold">Open Electricity Market.</h1>
            <p class="py-6">Discover a revolutionary platform where you can buy and sell electricity in real-time.</p>
            <p>{errormessage}</p>
          </div>
          <div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
            <form class="card-body">
              <h2 class="text-base font-semibold"> Log in to your account </h2>


              <div class="form-control mt-4">
                <input type="email" placeholder="Email" class="input input-bordered" required bind:value={email}/>

                <input type="password" placeholder="Password" class="input input-bordered" required bind:value={password}/>
                <!-- <label class="label" for="">
                  <a href="#" class="label-text-alt link link-hover">Forgot password?</a>
                </label> -->
              </div>

              {#if errormessage != ''}
                <h2 class="text-base font-semibold text-black bg-error rounded"> { errormessage } </h2>
              {/if}

              <div class="form-control mt-6">
                <button class="btn btn-primary" on:click={login}>Login</button>
              </div>
              <div class="form-control mt-3"> 
                <button class="btn btn-primary" on:click={signup}>Create an account</button>
              </div>
            </form>
          </div>
        </div>
      </div>
</main>