<script>
    import logo from '$lib/assets/Power2.svg';
    import pylonBack from '$lib/assets/pylonBack.png';
    //import Button from "../signup/signup.svelte"
    //let pylonBack = '$lib/assets/pylonBack.png';
    let username = '';
    let password = '';
</script>
<div class = "background">
    <img alt = "Background" src = {pylonBack}/>
</div>
<div class = "body">
    
    <img class= "logo" alt = "Logo" src = {logo}/>
    <h1>Welcome to Open Electricity Market</h1>
    <p>Login into account:</p>
    <input bind:value={username} placeholder="Username here..."/>
    <input bind:value={password} placeholder="Password here..."/>
    <Button class = "login">Login</Button>
    <Button class = "signup">Signup</Button>
</div>

<style>
    .body{
        text-align: center;
        display: flex;
        flex-flow: column;
        justify-content: center;
        position: absolute;
        height: 100%;
        width: 100%;
        align-items: center;
        color: white;
        /*background-image: url('$lib/assets/pylonBack.png');*/
    }
    h1{
        font-size: 50px
    }
    .background{
        position: absolute;
        height: 100%;
        width: 100%;
        top: 0;
        z-index: -1;
        background-size: cover;
    
    }
    .logo{
        height: 284px;
        width: 220px;
        display: flex;
        flex-flow: column;
        justify-content: center;
    }

    
</style>