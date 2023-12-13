<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<h1>Courses</h1>

<div class="courses">
	{#each data.courses as course}
		<div class="course">
			<a class="course-name" href="/grades/{data.student_id}/{data.year_id}/{course.id}">{course.name}</a>
			<p class="teacher">{course.teacher}</p>
			<div class="attendence">
				{#if course.absences}
					<span>Absences: {course.absences}</span>
				{/if}
				{#if course.tardies}
					<span>Tardies: {course.tardies}</span>
				{/if}
				{#if !course.absences && course.tardies}
					<span>🎉</span>
				{/if}
			</div>
		</div>
	{/each}
</div>

<style lang="less">
	.courses {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.course {
		background-color: var(--bg-1);
		border-radius: var(--border-radius);
		padding: 1rem;
	}

	.course-name {
		font-size: 1.4rem;
		margin: 0;
	}
	
	.teacher {
		margin-top: 0.4rem;
		margin-bottom: 0.4rem;
	}
	
	.attendence {
		margin: 0;
		margin-top: 0.4rem;
		color: var(--fg-1);
	}
</style>
