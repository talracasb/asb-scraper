<script lang="ts">
	import type { Schedule } from '$src/app';

	export let data: Schedule;
</script>

<h1>{data.student} - {data.homeroom}</h1>

<table>
	<tr>
		{#each data.header as period}
			{#if period.visible}
				<th>{period.name}<br />{period.time}</th>
			{/if}
		{/each}
	</tr>
	{#each data.days as day}
		<tr>
			{#each day as dayClass, i}
				{#if i != 2 && i != 5}
					<td>
						{#if dayClass}
							{dayClass.name}
						{/if}
					</td>
				{/if}
			{/each}
		</tr>
	{/each}
</table>

<style>
    td {
        background-color: aliceblue;
    }
    
    th {
        background-color: antiquewhite;
    }
</style>