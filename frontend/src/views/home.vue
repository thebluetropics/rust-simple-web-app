<template>
  <div class="text" v-if="state == 'unauthorized'">
    <span>You have to <router-link to="/auth/login">login</router-link> or <router-link to="/auth/register">register</router-link> in order to use the app.</span>
  </div>
  <div class="text" v-if="state == 'authorized'">
    <span>Welcome, <b>{{ profile.username }}</b>!</span>
  </div>
  <div class="actions" v-if="state == 'authorized'">
    <button @click="logout">Logout</button>
  </div>
</template>

<script>
import Cookies from "js-cookie"
import axios, { AxiosError } from "axios"

export default {
  data() {
    return {
      state: null,
      profile: {
        username: null,
        displayName: null
      }
    }
  },
  async mounted() {
    const token = Cookies.get("auth.token")

    if (!token) {
      this.state = "unauthorized"
      return
    }

    try {
      let res = await axios.get("/api/profile", {
        headers: {
          "Authorization": "Bearer " + token
        }
      })

      this.profile.username = res.data.username
      this.state = "authorized"
    } catch(err) {
      if (err instanceof AxiosError) {
        if (err.response.status == 401 && err.response.data.error == "token_expired") {
          console.error("Err: token expired.")
          this.state = "unauthorized"
          return
        }

        if (err.response.status == 401) {
          console.error("Err: token invalid.")
          this.state = "unauthorized"
          return
        }

        console.error("Err: unknown error")
      }
    }
  },
  methods: {
    async logout() {
      Cookies.remove("auth.token")
      this.state = "unauthorized"
    }
  }
}
</script>

<style scoped>
.text, .text * {
  font-size: 14px;
  font-weight: 400;
  color: #242424;
}

.text b {
  font-weight: 600;
}

.actions {
  margin-top: 5px;
}

.actions button, .text a {
  text-decoration: none;
  border: none;
  outline: none;
  background: none;
  font-size: 10px;
  font-weight: 600;
  padding: 3px 5px;
  background-color: #242424;
  color: #cfcfcf;
  border-radius: 5px;
  cursor: pointer;
  user-select: none;
}

.text a:visited, .text a:active {
  color: #cfcfcf;
}

.actions button:hover, .text a:hover {
  background-color: #404040;
}

.actions button:active, .text a:active {
  background-color: #cfcfcf;
  color: #404040;
}
</style>
