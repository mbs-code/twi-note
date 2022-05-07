import { createRouter, createWebHistory } from 'vue-router'
import Home from './pages/Home.vue'
import Timeline from './pages/Timeline.vue'
import Tag from './pages/Tag.vue'
import Phrase from './pages/Phrase.vue'
import About from './pages/About.vue'

const routes = [
  {
    path: '/',
    name: 'home',
    component: Home,
  },
  {
    path: '/timeline',
    name: 'timeline',
    component: Timeline,
  },
  {
    path: '/tag',
    name: 'tag',
    component: Tag,
  },
  {
    path: '/phrase',
    name: 'phrase',
    component: Phrase,
  },

  // TODO: 未実装
  {
    path: '/about',
    name: 'about',
    component: About,
  },
]

const router = createRouter({
  history: createWebHistory('/'),
  routes,
})

export default router

export const menuRoutes = routes
