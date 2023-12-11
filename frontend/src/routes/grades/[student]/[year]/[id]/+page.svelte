<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div>
	{#each data.assignments as assignment, i}
		<h1>{assignment.name}</h1>
	{/each}
</div>
