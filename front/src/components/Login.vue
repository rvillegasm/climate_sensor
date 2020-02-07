<template>
    <div class='login'>
        <h3>Sign In</h3>
        <p>
            Please, use your Google account <br> <br>
            <button class="social-button" @click="socialLogin">
                <img src="../assets/google-logo.png" alt="Google Logo">
            </button>
        </p>
    </div>
</template>

<script>
import firebase from 'firebase';

export default {
    name: 'Login',
    methods: {
        socialLogin() {
            const provider = new firebase.auth.GoogleAuthProvider;

            firebase.auth().signInWithPopup(provider).then((result => {
                this.$router.replace('sensors');
            })).catch((err => {
                alert('Something went wrong with the sign in process. ' + err.message)
            }));
        }
    }
}
</script>

<style scoped>

.social-button {
    width: 75px;
    background: white;
    padding: 10px;
    border-radius: 100%;
    box-shadow: 0 2px 4px 0 rgba(0, 0, 0, 2);
    outline: 0;
    border: 0;
}

.social-button:active {
    box-shadow: 0 2px 4px 0 rgba(0, 0, 0, 1);
}

.social-button img {
    width: 100%;
}

</style>