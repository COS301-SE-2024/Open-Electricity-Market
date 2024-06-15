<script>
    import logo from '$lib/assets/Logo.png';
    import {goto} from "$app/navigation";
    function signup(){
        goto("/signup");
    }
    function login(){
        //Username and password to be validated.
        //send to main page
        goto("/Main/Dashboard");
    }
    let username = '';
    let password = '';
</script>

<div class = "h-screen bg-cover bg-no-repeat" style="background-image: url('../src/lib/assets/pylonBack.png');">
    <div class = "bg-black/95 h-screen grid gap-0 grid-cols-1 grid-rows-7 content-center">
        <img class= "h-auto max-w-full" alt = "Logo" src = {logo}/>
        <h1 class ="mb-4 text-heading1 font-heading1">Welcome To OpenElectricityMarket!</h1>
        <h2 class = "mb-4 text-heading2 font-heading2">Log In</h2>
        <input type = "text" placeholder="Username" class = "input bg-fillin text-fillinText font-fillin w-fillin h-fillin"  bind:value={username}/>
        <input type = "password" placeholder="Password" class = "input bg-fillin text-fillinText font-fillin w-fillin h-fillin" bind:value={password}/>
        <button class = "btn bg-btn btn-outline text-btnTxt w-btn h-btn" on:click={login}>Login</button>
        <button class = "btn bg-btn btn-outline text-btnTxt w-btn h-btn" on:click={signup}>Signup</button>
    </div>
</div>

<style>
    @font-face{
        font-family: 'NATS';
        font-style: normal;
        font-weight: 50;
        src: url("/fonts/NATS-Regular.ttf");
    }
    .background{
        position: absolute;
        top: 0;
        left: 0;
        height: 100%;
        width: 100%;
        background-image: url('$lib/assets/pylonBack.png');
        background-size: cover;
    }
    
    .body
    {
        color: white;
        display: flex;
        position: absolute;
        flex-flow: column;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.93);
        align-items: center;
    }
    .logo{
        position: relative;
        top: 20px;
    }
    /*h1{
        font-weight: normal;
        font-size: 50px;
        top: 24px;
        position: relative;
        font-family: NATS;
        letter-spacing: 9px;
    }*/
    /*h2{
        font-weight: normal;
        font-size: 33px;
        top: 22px;
        position: relative;
        font-family: NATS;
        letter-spacing: 10px;
    }*/
    .fillin{
        height: 43px;
        width: 378px;
        margin: 18px;
        position: relative;
        top: 27px;
        border-radius: 6px;
        background-color: white;
        font-size:27px;
        color: black;
    }
    ::placeholder{
        color: black;
    }
    ::-ms-input-placeholder{
        color: black;
    }
    /*.buttons{
        background-color: #124E86;
        border: black;
        border-radius: 6px;
        color: white;
        margin: 15px;
        position: relative;
        top: 49px;
        text-align: center;
        font-size: 28px;
        height: 66px;
        width: 336px;
        transition-duration: 0.4s;
    }
    .buttons:hover{
        background-color: black;
        border: #124E86;
        border-radius: 6px;
        color: white;
        margin: 15px;
        position: relative;
        top: 49px;
        text-align: center;
        font-size: 28px;
        height: 66px;
        width: 336px;
        transition-duration: 0.4s;
    }*/
    
</style>