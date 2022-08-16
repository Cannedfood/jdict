<script lang="ts">
import { SearchService } from "../SearchService";
import { debounce } from 'lodash';
import { push as pushRoute } from 'svelte-spa-router'

export let searchTerm = "";

const searchService = new SearchService();

let completions = [] as string[];

const fetchCompletions = debounce(async() => completions = await searchService.autocomplete(searchTerm), 100);
function updateCompletions() {
    completions = completions.filter(c => c.includes(searchTerm));
    fetchCompletions();
}

function keydown(e: KeyboardEvent) {
    if(e.key == 'Enter' && searchTerm.trim() !== '') {
        pushRoute(`/search/${searchTerm}`)
    }
}

</script>

<div class="search-bar">
    <div class="search">
        <button>✍️</button>
        <button>部</button>
        <div class="divider"></div>
        <input type="text"
            name="searchbox" id="searchbox"
            placeholder="Search..."
            bind:value={searchTerm}
            on:input={updateCompletions}
            on:blur={() => completions = []}
            on:keydown={keydown}
        >
        <button>🔍</button>
    </div>
    {#if completions.length > 0}
        <div class="auto-completions">
            {#each completions as completion}
                <div>{completion}</div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .search-bar {
        position: relative;
        width: fit-content;
        font-size: 2rem;
    }
    .search {
        display: flex;
        flex-flow: nowrap row;

        height: 2.5rem;
        border: 1px solid white;
        border-radius: .5rem;
    }
    .search:focus-within {
        outline: 2px solid black;
    }
    .search>* {
        box-sizing: border-box;
        height: 100%;
        border: none;
        margin: 0;
        font-size: inherit;
        outline: none;
    }
    .divider {
        min-width: .2em;
        background: black;
    }

    .auto-completions {
        display: absolute;
        bottom: -100%;
        box-shadow: .1em .1em .1em .1em #0002;
        padding: 1em;
    }
</style>