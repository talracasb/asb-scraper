<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<h1>{data.day}</h1>

<p>Welcome {data.student_name}</p>

<a href="/grades">Assignment Grades</a>
<a href="/schedule">Schedule</a>
<a href="/lunch">Lunch Menu</a>
