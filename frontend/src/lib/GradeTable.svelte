<script lang="ts">
	import type { Standard } from "$src/app";

    export let standards: Standard[];
</script>

<table>
    <tr>
        <th class="standard-header">Standard</th>
        <th class="grade-header" colspan="4">Grade</th>
    </tr>
    {#each standards as standard}
        <tr>
            <td>{@html standard.name}</td>
            {#each {length: 4} as _, i}
                <td class="grade-block {standard.grade == i+1 ? "grade-filled" : "grade-unfilled"}" colspan=1>{i+1}</td>
            {/each}
        </tr>
    {/each}
    <!-- {#if standards.length > 0}
        <th colspan="5">Learning Behaviours</th>
        {#each assignment.learning_behaviours as standard}
            <tr>
                <td>{@html standard.name}</td>
                {#each {length: 4} as _, i}
                    <td class="grade-block {standard.grade == i+1 ? "grade-filled" : "grade-unfilled"}" colspan=1>{i+1}</td>
                {/each}
            </tr>
        {/each}
    {/if} -->
</table>

<style>
	table {
		width: 100%;
		table-layout: fixed;
	}

	.grade-header, .grade-block {
		width: 8.1rem;
		text-align: center;
	}

	.grade-filled {
		background-color: var(--blue);
		color: var(--bg-0);
	}

	.standard-header, .grade-header {
		text-align: left;
	}
</style>