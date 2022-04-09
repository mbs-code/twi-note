import { createApp } from 'vue'
import naive from 'naive-ui'
import router from './router'
import App from './App.vue'
import './assets/css/index.scss'

createApp(App).use(naive).use(router).mount('#app')
