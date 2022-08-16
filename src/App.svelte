<script lang="ts">
    import Router, { location } from 'svelte-spa-router'
    import SearchBar from './lib/SearchBar.svelte'
    import Home from './routes/Home.svelte'
    import Results from './routes/Results.svelte'

    const routes = {
        "/": Home,
        "/search/:searchTerm": Results
    };
</script>

<nav>
    <a href="/#/">Home</a>
    <a href="/#/search/karasu">Search</a>

    <div class="search-container" class:center-page={$location == '/'}>
        <SearchBar />
    </div>
</nav>

<main>
    <Router routes={routes}/>
</main>

<style lang="scss">
.search-container {
    width: fit-content;
    margin-left: auto;
    margin-right: auto;
    transition: 300ms margin-top;
    &.center-page {
        margin-top: 40vh;
    }
}
</style>