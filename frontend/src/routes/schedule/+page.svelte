<script lang="ts">
	import type { Schedule } from '$src/app';

	export let data: Schedule;
</script>

<div class="table-container">
	<h1 class="title">{data.student} ({data.homeroom})</h1>

	<table>
		<tr>
			<th class="day"></th>
			{#each data.header as period}
				{#if period.visible}
					<th>
						<p class="period-name">{period.name}</p>
						<p class="period-time">{period.time}</p>
					</th>
				{/if}
			{/each}
		</tr>
		{#each data.days as day, i}
			<tr>
				<th class="day">Day {i + 1}</th>
				{#each day as dayClass, i}
					{#if i != 2 && i != 5}
						{#if dayClass}
							<td class="asb-schedule-color-{dayClass.color}">
								{dayClass.name}
							</td>
						{:else}
							<td class="unknown">?</td>
						{/if}
					{/if}
				{/each}
			</tr>
		{/each}
	</table>
</div>

<style lang="less">
	@import (css) '$css/schedule.css';

	.unknown {
		color: var(--fg);
		text-align: center;
	}

	.title {
		text-align: center;
		font-size: 2.5rem;
	}

	td,
	th {
		height: 4rem;
		width: 10rem;
	}

	td {
		color: var(--bg-0);
	}

	.day {
		width: 4rem;
	}

	.table-container {
		display: flex;
		flex-direction: column;
	}

	.period-name {
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
		font-size: 1.4rem;
	}

	.period-time {
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
		font-size: 1rem;
	}
</style>
