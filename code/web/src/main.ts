import './main.scss'

import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'

import App from './App.vue'
import Home from './routes/Home.vue'
import Search from './routes/Search.vue'
import KanjiSearch from './routes/KanjiSearch.vue'

import { SearchService } from './backend/search'

createApp(App)
.provide('search-service', new SearchService('/api'))
.use(createRouter({
	history: createWebHashHistory(),
	routes: [
		{ path: '/', component: Home, name: 'home' },
		{ path: '/search', redirect: '/' },
		{ path: '/search/:query', component: Search, name: 'search' },
		{ path: '/kanji-grid/:query?', component: KanjiSearch, name: 'kanji-grid' },
	]
}))
.mount('#app')
