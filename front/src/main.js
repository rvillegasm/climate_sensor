// The Vue build version to load with the `import` command
// (runtime-only or standalone) has been set in webpack.base.conf with an alias.
import Vue from 'vue'
import App from './App'
import router from './router'
import firebase from 'firebase'

Vue.config.productionTip = false

// Your web app's Firebase configuration
var firebaseConfig = {
  apiKey: "AIzaSyBTizv_2hmz8-wyvCPtXWx3cRxTNB5LYTk",
  authDomain: "telem-0.firebaseapp.com",
  databaseURL: "https://telem-0.firebaseio.com",
  projectId: "telem-0",
  storageBucket: "telem-0.appspot.com",
  messagingSenderId: "409511879424",
  appId: "1:409511879424:web:9781d5d49afb919c97c9e5"
};
// Initialize Firebase
firebase.initializeApp(firebaseConfig);


/* eslint-disable no-new */
new Vue({
  el: '#app',
  router,
  components: { App },
  template: '<App/>'
})
