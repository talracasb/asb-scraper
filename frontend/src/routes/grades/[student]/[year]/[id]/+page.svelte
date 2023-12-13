<script lang="ts">
	import type { Course } from '$src/app';
	import GradeTable from '$src/lib/GradeTable.svelte';

	export let data: Course;
</script>

<div>
	<div class="course-info">
		<h1 class="name">{data.name}</h1>
		<h2 class="teacher">{data.teacher}</h2>
		<div class="attendence">
			{#if data.absences}
				<span>Absences: {data.absences}</span>
			{/if}
			{#if data.tardies}
				<span>Tardies: {data.tardies}</span>
			{/if}
			{#if !data.absences && data.tardies}
				<span>🎉</span>
			{/if}
		</div>
	</div>

	<div class="assignments">
		{#each data.assignments as assignment, i}
			<div class="assignment">
				<h2 class="assignment-name">{@html assignment.name}</h2>
				{#if assignment.reporting_categories.length > 0}	
					<h3>Reporting Categories</h3>
					<GradeTable standards={assignment.reporting_categories} />
				{/if}
				{#if assignment.learning_behaviours.length > 0}
					<h3>Learning Behaviours</h3>
					<GradeTable standards={assignment.learning_behaviours} />
				{/if}
			</div>

			{#if i != data.assignments.length-1}
				<div class="line" />
			{/if}
		{/each}
	</div>
</div>

<style lang="less">
	.line {
		background-color: var(--bg-4);
		width: auto;
		height: 1px;
		margin-top: 0.4rem;
	}

	.course-info {
		display: flex;
		align-items: center;
		flex-direction: column;
		margin-bottom: 3rem;
	}

	.name {
		font-size: 2.5rem;
		text-align: center;
		margin-top: 2rem;
		margin-bottom: 0.5rem;
	}

	.teacher {
		margin: 0;
		margin-bottom: 0.4rem;
		font-size: 1.8rem;
	}

	.attendence {
		color: var(--fg-1);
	}

	.assignments {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.assignment-name {
		margin-top: 0;
		margin-bottom: 0.7rem;
		font-size: 2rem;
	}

	h3 {
		margin-top: 0.7rem;
		margin-bottom: 0.7rem;
	}
</style>
