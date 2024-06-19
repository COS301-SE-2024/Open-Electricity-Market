<script>
    import logo from '$lib/assets/Logo.png';
    import {goto} from "$app/navigation";
    let email = '';
    let password = '';
    function signup(){
        goto("/signup");
    }
    function login(){
        //Username and password to be validated.
        //send to main page
        goto("/Main/Dashboard");
    }
</script>

<main>
    <div class="hero min-h-screen" style="background-image: url(https://images.unsplash.com/photo-1510595256055-e44b27efe497?q=80&w=1700&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D);">
        <div class="hero-overlay bg-opacity-60 "></div>
        <div class="hero-content flex-col lg:flex-row-reverse ">
          <div class="text-center lg:text-left">
            <h1 class="text-5xl font-bold">Open Electricity Market.</h1>
            <p class="py-6">Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi.</p>
          </div>
          <div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
            <form class="card-body">
              <h2 class="text-base font-semibold"> Log in to your account </h2>

              <div class="form-control mt-1">
                <input type="email" placeholder="Email" class="input input-bordered" required bind:value={email}/>
              </div>
              <div class="form-control mt-4">
                <input type="password" placeholder="Password" class="input input-bordered" required bind:value={password}/>
                <!-- <label class="label" for="">
                  <a href="#" class="label-text-alt link link-hover">Forgot password?</a>
                </label> -->
              </div>
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