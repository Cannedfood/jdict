import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { VitePWA } from 'vite-plugin-pwa'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
	resolve: {
		alias: {
			'@': path.resolve(__dirname, './src')
		}
	},
	plugins: [
		vue(),
		VitePWA({
			registerType: 'autoUpdate',
			injectRegister: 'inline',
			workbox: {
				globPatterns: []
			},
			devOptions: {
				enabled: false
			},
			manifest: {
				id: "jdict",
				name: "jDict",
				short_name: "jDict",
				description: "A Japanese Dictionary",

				scope: '/',
				orientation: 'natural',

				display: "standalone",
				background_color: "#242424",
				theme_color: "purple"
			}
		})
	],
	server: {
		proxy: {
			'/api': {
				target: 'http://127.0.0.1:8000'
			}
		}
	}
})
