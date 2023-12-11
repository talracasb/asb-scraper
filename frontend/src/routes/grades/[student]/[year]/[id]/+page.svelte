<script lang="ts">
	import type { Course } from '$src/app';

	export let data: Course;
</script>

<div>
    <h1>{@html data.name} ({@html data.teacher})</h1>
    <p>Absences: {data.absences} Tardies: {data.tardies}</p>
	{#each data.assignments as assignment}
		<h2>{@html assignment.name}</h2>
        {#if assignment.reporting_categories.length > 0}
            <h3>Reporting Categories</h3>
            {#each assignment.reporting_categories as standard}
                <p>{@html standard.name} - {standard.grade}</p>
            {/each}
        {/if}
        {#if assignment.learning_behaviours.length > 0}
            <h3>Learning Behaviours</h3>
            {#each assignment.learning_behaviours as standard}
                <p>{@html standard.name} - {standard.grade}</p>
            {/each}
        {/if}
	{/each}
</div>
