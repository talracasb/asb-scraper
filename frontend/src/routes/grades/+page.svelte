<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<h1>Courses</h1>

<div>
	{#each data.courses as course}
		<div>
			<a href="/grades/{data.student_id}/{data.year_id}/{course.id}">{course.name}</a>
			<p>{course.teacher}</p>
			<p>Absences: {course.absences} Tardies: {course.tardies}</p>
		</div>
		<br />
	{/each}
</div>
