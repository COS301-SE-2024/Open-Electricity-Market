<script lang="ts">
    import logo from '$lib/assets/Logo.png';
    //import pylonBack from '$lib/assets/pylonBack.png';
    //import Button from "/mnt/c/University/Year3/Semester1/COS301/Capstone/CapstoneWork/Open-Electricity-Market/Frontend/src/routes/Button.svelte";
    //let pylonBack = '$lib/assets/pylonBack.png';
    import {goto} from "$app/navigation";
    function signup(){
        goto("/signup");
    }
    function login(){
        //Username and password to be validated.
        //send to main page
        //goto("");
    }
    let username = '';
    let password = '';
</script>

<div class = "background">
    <div class = "body">
        <img class= "logo" alt = "Logo" src = {logo}/>
        <h1>Welcome To OpenElectricityMarket!</h1>
        <h2>Login into account:</h2>
        <input class = "fillin" type = "text" color = "black" bind:value={username} placeholder="Username here..."/>
        <input class = "fillin" type = "password" size = "35" bind:value={password} placeholder="Password here..."/>
        <button class = "buttons" on:click={login}>Login</button>
        <button class = "buttons" on:click={signup}>Signup</button>
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
    h1{
        font-weight: normal;
        font-size: 50px;
        top: 24px;
        position: relative;
        font-family: NATS;
        letter-spacing: 9px;
    }
    h2{
        font-weight: normal;
        font-size: 33px;
        top: 22px;
        position: relative;
        font-family: NATS;
        letter-spacing: 10px;
    }
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
    .buttons{
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
    }
    
</style>