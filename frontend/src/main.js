import "./styles/global.css"

import App from "./app.vue"
import { createApp } from "vue"
import { createRouter, createWebHistory } from "vue-router"
import Home from "./views/home.vue"
import Login from "./views/login.vue"
import Register from "./views/register.vue"

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", component: Home },
    { path: "/auth/login", component: Login },
    { path: "/auth/register", component: Register }
  ]
})

const app = createApp(App)
app.use(router)
app.mount("#app")
