<script lang="ts">
	import Navbar from "$lib/Navbar.svelte";
</script>

<Navbar />

<main>
    <slot />
</main>

<style lang="less">
    main {
        padding: 1rem;
        padding-top: 5em;
    }
</style>