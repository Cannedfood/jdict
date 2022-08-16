import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'

import './main.scss'

import App from './App.vue'
import Home from './routes/Home.vue'
import Search from './routes/Search.vue'

import { SearchService } from './backend/search'

createApp(App)
.provide('search-service', new SearchService('/api'))
.use(createRouter({
	history: createWebHashHistory(),
	routes: [
		{ path: '/', component: Home, name: 'home' },
		{ path: '/search', redirect: '/' },
		{ path: '/search/:query', component: Search, name: 'search' }
	]
}))
.mount('#app')
