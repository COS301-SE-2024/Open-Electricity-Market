<script>
    import logo from '$lib/assets/Logo.png';
    import pylonBack from '$lib/assets/pylonBack.png';
    import Button from "/mnt/c/University/Year3/Semester1/COS301/Capstone/CapstoneWork/Open-Electricity-Market/Frontend/src/routes/Button.svelte";
    let username = '';
    let password = '';
</script>
<div class = "background">
    <img alt = "Background" src = {pylonBack}/>
</div>
<div class = "body">
    
    <img class= "logo" alt = "Logo" src = {logo}/>
    <h1>Create an account with us</h1>
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