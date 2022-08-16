<script lang="ts">
import { SearchService } from "../SearchService";

const searchService = new SearchService();

export let params = {
    searchTerm: ''
};

</script>

<div class="fade-in">
    <h1> Results </h1>

    <div class="tiles">
        <div>
            {params.searchTerm}
        </div>
    </div>
</div>


<style lang="scss">
@keyframes fade-in {
    from { opacity: 0%;   }
    to   { opacity: 100%; }
}
.fade-in {
    animation: fade-in 100ms ease-out;
}
.tiles {
    display: flex;
    flex-flow: row wrap;
    &>* {
        background: green;
        padding: 1em;
        cursor: pointer;
    }
}
</style>