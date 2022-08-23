<template lang="pug">
.spinner
</template>

<style scoped>
.spinner {
	display: block;
	width: 3em;
	height: 3em;
	border: .1em solid rgba(255,255,255,.3);
	margin-inline: auto;
	margin-block: 2em;
	border-radius: 50%;
	border-top-color: #3F3;
	animation: spin 1s ease-in-out infinite, appear 200ms;
}

@keyframes appear {
	0% { opacity: 0; }
	50% { opacity: 0; }
	100% { opacity: 100%; }
}

@keyframes spin {
	to { -webkit-transform: rotate(360deg); }
}
@-webkit-keyframes spin {
	to { -webkit-transform: rotate(360deg); }
}
</style>
