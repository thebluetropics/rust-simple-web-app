<template>
  <div class="form">
    <input ref="usernameField" type="text" placeholder="username"/>
    <input ref="passwordField" type="password" placeholder="password"/>
    <button @click="submit">Register</button>
  </div>
</template>

<script>
import "../styles/form.css"
import axios from "axios"
import Cookies from "js-cookie"

export default {
  methods: {
    async submit() {
      try {
        const response = await axios.post("/api/register", {
          username: this.$refs.usernameField.value,
          password: this.$refs.passwordField.value
        })
        
        Cookies.set("auth.token", response.data.token)
        this.$router.push("/")
      } catch(e) {
        if (e.response.status == 409 && e.response.data.error == "user_exists") {
          console.error("Err: user already exists.")
          return
        }

        if (e.response.status == 500) {
          console.error("Err: unknown server error")
          return
        }

        console.error("Err: unknown error.")
      }
    }
  }
}
</script>
