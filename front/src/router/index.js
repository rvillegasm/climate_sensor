import Vue from 'vue'
import Router from 'vue-router'
import Hello from '@/components/Hello'
import Login from '@/components/Login'
import Sensors from '@/components/Sensors'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      name: 'Hello',
      component: Hello
    },
    {
      path: '/login',
      name: 'Login',
      component: Login
    },
    {
      path: '/sensors',
      name: 'Sensors',
      component: Sensors,
      beforeEnter: (_, from, next) => {
        if (from.name === 'Login') {
          next();
        }
        else {
          next({
            name: 'Login'
          });
        }
      }
    }
  ],
  mode: 'history',
})
